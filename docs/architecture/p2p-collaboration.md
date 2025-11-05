<!-- neira:meta
id: NEI-20251105-p2p-collaboration
intent: docs
summary: |
  Архитектура P2P связи между экземплярами Neira для распределённого обучения
  и совместного решения задач.
-->

# P2P Collaboration Architecture

## Motivation

Пользователь может запускать Neira на нескольких устройствах (desktop, mobile, серверы). В простое эти экземпляры должны:
1. Обмениваться задачами и данными
2. Совместно обучаться на распределённых датасетах
3. Делиться результатами анализа и новыми органами

**Цель**: Превратить множество изолированных Neira в распределённую «нервную систему».

## Architecture Overview

```
┌────────────────────────────────────────────────────────────┐
│                    Discovery Layer                         │
│  (mDNS/Bonjour для LAN, DHT для WAN)                     │
└────────────────────────────────────────────────────────────┘
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
    ┌─────▼─────┐     ┌─────▼─────┐     ┌─────▼─────┐
    │  Neira 1  │     │  Neira 2  │     │  Neira 3  │
    │  Desktop  │◄────┤   Mobile  │────►│   Server  │
    │ Win/Linux │     │  Android  │     │   Linux   │
    └───────────┘     └───────────┘     └───────────┘
         │                  │                  │
         │                  │                  │
    ┌────▼──────────────────▼──────────────────▼────┐
    │           Distributed Task Queue              │
    │  (Redis/RabbitMQ или libp2p gossipsub)       │
    └───────────────────────────────────────────────┘
         │                  │                  │
    ┌────▼─────┐       ┌────▼─────┐      ┌────▼─────┐
    │ Training │       │ Organ    │      │ Analysis │
    │   Task   │       │  Build   │      │   Task   │
    └──────────┘       └──────────┘      └──────────┘
```

## Core Components

### 1. Discovery Service

**Задача**: Находить другие экземпляры Neira в сети.

**Протоколы**:
- **LAN**: mDNS (Bonjour) — zero-config discovery
  ```rust
  // Использовать mdns crate
  use mdns_sd::{ServiceDaemon, ServiceInfo};
  
  let mdns = ServiceDaemon::new()?;
  let service = ServiceInfo::new(
      "_neira._tcp.local.",
      "neira-desktop-1",
      "192.168.1.100",
      9090,
      &[("version", "1.0.0")]
  )?;
  mdns.register(service)?;
  ```

- **WAN**: Distributed Hash Table (DHT) через libp2p
  ```rust
  use libp2p::{Swarm, kad::Kademlia};
  
  let mut swarm = Swarm::new(...);
  swarm.behaviour_mut().add_address(&peer_id, addr);
  ```

**Безопасность**:
- Whitelist доверенных peer ID (cryptographic keys)
- Опциональный password/token для join

### 2. P2P Communication Layer

**Протокол**: libp2p (Rust-native, production-ready)

**Features**:
- Multiplexing (несколько streams через одно соединение)
- NAT traversal (через STUN/TURN)
- Encryption (Noise protocol)
- PubSub (gossipsub для broadcast сообщений)

**Пример**:
```rust
use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent},
    identity::Keypair,
    PeerId,
    Swarm,
};

// Создание identity
let local_key = Keypair::generate_ed25519();
let local_peer_id = PeerId::from(local_key.public());

// Topic для задач
let task_topic = gossipsub::IdentTopic::new("neira-tasks");

// Broadcast задачи
swarm.behaviour_mut()
    .publish(task_topic.clone(), task_json.as_bytes())?;

// Получение задач
match swarm.select_next_some().await {
    GossipsubEvent::Message { message, .. } => {
        let task: Task = serde_json::from_slice(&message.data)?;
        // Обработать task
    }
}
```

### 3. Distributed Task Queue

**Задача**: Распределять задачи между экземплярами с учётом их возможностей.

**Типы задач**:
1. **Training Task** — обучение на датасете
2. **Organ Build** — компиляция нового органа
3. **Analysis Task** — анализ данных
4. **Idle Task** — фоновые задачи в простое

**Task Schema**:
```rust
#[derive(Serialize, Deserialize)]
struct DistributedTask {
    id: String,
    task_type: TaskType,
    priority: u8,
    requirements: Requirements,
    payload: serde_json::Value,
    created_at: DateTime<Utc>,
    deadline: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
struct Requirements {
    min_cpu_cores: u8,
    min_ram_gb: u16,
    gpu_required: bool,
    estimated_duration_sec: u64,
}
```

**Алгоритм распределения**:
1. Neira broadcast свои capabilities (CPU/RAM/GPU)
2. При получении задачи каждая Neira проверяет `requirements`
3. Если подходит — отправляет "bid" с оценкой времени выполнения
4. Создатель задачи выбирает лучший bid и отправляет задачу
5. Исполнитель отчитывается о прогрессе через gossipsub

### 4. Distributed Training

**Цель**: Обучать модели параллельно на нескольких устройствах.

**Подходы**:

#### A. Data Parallelism (рекомендуется для MVP)
- Каждая Neira обучается на своей части датасета
- Периодически синхронизируют градиенты/веса
- Усреднение весов (Federated Averaging)

```rust
// Псевдокод
struct FederatedTraining {
    local_model: Model,
    global_weights: Weights,
}

impl FederatedTraining {
    async fn train_epoch(&mut self, local_data: &Dataset) {
        // 1. Обучение на локальных данных
        self.local_model.train(local_data);
        
        // 2. Отправка градиентов координатору
        let gradients = self.local_model.get_gradients();
        broadcast_gradients(gradients).await;
        
        // 3. Получение усреднённых весов
        self.global_weights = receive_averaged_weights().await;
        self.local_model.set_weights(self.global_weights);
    }
}
```

#### B. Model Parallelism (для больших моделей)
- Модель разделена на части между устройствами
- Каждая Neira держит свой слой/модуль
- Forward/backward pass через сеть

**Координатор**:
- Самая мощная Neira или выделенный сервер
- Агрегирует результаты
- Хранит checkpoint'ы

### 5. Organ Sharing

**Задача**: Делиться новыми органами между экземплярами.

**Протокол**:
1. Neira создаёт новый орган
2. Запускает тесты локально
3. Публикует OrganTemplate + wasm/binary в DHT
4. Другие Neira скачивают и проверяют (иммунная система)
5. При success — устанавливают орган

**Schema**:
```rust
#[derive(Serialize, Deserialize)]
struct SharedOrgan {
    template: OrganTemplate,
    binary: Vec<u8>,  // wasm или native lib
    checksum: String,  // SHA256
    signature: String, // Ed25519 signature от создателя
    test_results: Vec<TestResult>,
    created_by: PeerId,
    created_at: DateTime<Utc>,
}
```

**Безопасность**:
- Цифровая подпись от доверенного peer
- Sandbox execution (wasm)
- Лимфатический фильтр проверяет на вредоносность
- Quarantine перед установкой

### 6. Idle Task Distribution

**Задача**: Использовать простой CPU/GPU для полезных вычислений.

**Примеры задач**:
- Переобучение старых моделей на новых данных
- Валидация органов других Neira
- Анализ логов для паттернов
- Генерация синтетических датасетов

**Anti-Idle Service**:
```rust
struct IdleDetector {
    cpu_threshold: f32,  // < 20% → idle
    gpu_threshold: f32,  // < 10% → idle
    idle_duration: Duration,  // 5 min
}

impl IdleDetector {
    async fn check_and_request_task(&self) {
        if self.is_idle().await {
            let task = request_idle_task_from_peers().await;
            if let Some(task) = task {
                execute_task(task).await;
            }
        }
    }
}
```

## Network Topology

### Star (начальный вариант)
```
     [Coordinator]
      /    |    \
   [N1]  [N2]  [N3]
```
- Простая реализация
- Single point of failure
- Подходит для малого количества узлов

### Mesh (целевой вариант)
```
[N1]───[N2]
  │   ╱  │
  │ ╱    │
[N3]───[N4]
```
- Децентрализация
- Fault tolerance
- Масштабируемость

**Переход**: Star для MVP (1-3 устройства), Mesh когда >3.

## Security Considerations

### 1. Authentication
- Ed25519 keypairs для каждой Neira
- Whitelist доверенных peer IDs в config
- Опциональный shared secret для LAN

### 2. Authorization
- Role-based: owner, collaborator, guest
- Owner может broadcast задачи
- Collaborator может выполнять и создавать
- Guest только получает результаты (read-only)

### 3. Data Privacy
- Датасеты не покидают устройство (только градиенты)
- Шифрование всех P2P сообщений (Noise protocol)
- Опциональная анонимизация метрик

### 4. DOS Protection
- Rate limiting для задач от peer'а
- Reputation system (блокировка плохих peer'ов)
- Proof-of-work для дорогих операций (опционально)

## Implementation Plan

### Phase 1: Discovery & Communication (Week 1-2)
- [ ] Интеграция libp2p в backend
- [ ] mDNS service discovery для LAN
- [ ] Базовый gossipsub (broadcast/receive messages)
- [ ] UI: список обнаруженных peers

### Phase 2: Task Distribution (Week 3-4)
- [ ] Task queue implementation
- [ ] Capability advertising
- [ ] Bidding system
- [ ] Task assignment и tracking

### Phase 3: Distributed Training (Week 5-6)
- [ ] Federated learning framework
- [ ] Gradient aggregation
- [ ] Model checkpoint sync
- [ ] Progress monitoring

### Phase 4: Organ Sharing (Week 7-8)
- [ ] Organ publish/subscribe
- [ ] Signature verification
- [ ] Sandbox execution (wasm)
- [ ] Auto-update mechanism

## Configuration

```toml
# config/p2p.toml
[discovery]
enabled = true
mdns_enabled = true  # LAN discovery
dht_enabled = false  # WAN discovery (для production)

[network]
listen_addr = "/ip4/0.0.0.0/tcp/0"  # Random port
bootstrap_peers = [
    # Для WAN
    # "/ip4/server.example.com/tcp/4001/p2p/QmBootstrapPeer"
]

[security]
keypair_path = "data/p2p_keypair.json"
trusted_peers = [
    # "QmPeerID1",
    # "QmPeerID2",
]
require_authentication = true

[tasks]
max_concurrent = 2
idle_threshold_cpu = 0.2  # 20%
idle_threshold_gpu = 0.1  # 10%
idle_duration_sec = 300   # 5 min

[training]
sync_interval_sec = 60    # Sync weights every minute
checkpoint_interval_sec = 600  # Save checkpoint every 10 min
```

## Metrics

Новые метрики для P2P:
```
neira_p2p_peers_total          # Количество подключённых peers
neira_p2p_tasks_received_total # Получено задач от peers
neira_p2p_tasks_sent_total     # Отправлено задач peers
neira_p2p_training_syncs_total # Синхронизаций весов модели
neira_p2p_organs_shared_total  # Поделенных органов
neira_p2p_bandwidth_bytes      # Сетевой трафик
neira_p2p_latency_ms           # Задержка до peers
```

## Testing Strategy

### Local Testing
1. Запустить 3 экземпляра backend на разных портах
2. Проверить discovery через mDNS
3. Отправить task от одного к другому
4. Проверить синхронизацию весов модели

### Integration Testing
```rust
#[tokio::test]
async fn test_peer_discovery() {
    let neira1 = spawn_neira_instance(9090).await;
    let neira2 = spawn_neira_instance(9091).await;
    
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    let peers1 = neira1.list_peers().await;
    assert_eq!(peers1.len(), 1);
    assert_eq!(peers1[0].id, neira2.peer_id());
}

#[tokio::test]
async fn test_task_distribution() {
    let neira1 = spawn_neira_instance(9090).await;
    let neira2 = spawn_neira_instance(9091).await;
    
    let task = Task::new(TaskType::Analysis, json!({"data": "test"}));
    neira1.broadcast_task(task.clone()).await;
    
    let received = neira2.receive_task().await;
    assert_eq!(received.id, task.id);
}
```

## Future Enhancements

### Phase 2+ Features
- [ ] Cross-platform P2P (Desktop ↔ Mobile)
- [ ] WAN support через relay servers
- [ ] Tor/I2P для анонимности
- [ ] Incentive mechanism (токены за выполнение задач)
- [ ] Public task marketplace
- [ ] Automatic load balancing
- [ ] Peer reputation system
- [ ] Distributed storage (IPFS integration)

## References

- [libp2p Documentation](https://docs.libp2p.io/)
- [Federated Learning Paper](https://arxiv.org/abs/1602.05629)
- [Gossipsub Specification](https://github.com/libp2p/specs/tree/master/pubsub/gossipsub)
- [Noise Protocol Framework](https://noiseprotocol.org/)
- [mDNS RFC 6762](https://tools.ietf.org/html/rfc6762)

---

**Статус**: Архитектурный документ для Phase 2-4 реализации  
**Зависит от**: Phase 1 (Backend API stabilization)  
**Критичность**: HIGH (core feature для MVP)

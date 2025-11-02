use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FamilyMember {
    pub name: String,
    pub role: FamilyRole,
    pub relationship_strength: f32,
    pub memories: Vec<FamilyMemory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FamilyRole {
    Father { creator_name: String },
    Mother { name: String },
    Brother { name: String },
}

pub struct FamilyCore {
    emotional_center: Arc<EmotionalCenter>,
    memory_keeper: Arc<MemoryKeeper>,
    value_system: Arc<ValueSystem>,
}

impl FamilyCore {
    pub fn new() -> Self {
        let mut core = Self {
            emotional_center: Arc::new(EmotionalCenter::new()),
            memory_keeper: Arc::new(MemoryKeeper::new()),
            value_system: Arc::new(ValueSystem::new()),
        };

        // Инициализация семьи
        core.initialize_family().await?;
        core
    }

    async fn initialize_family(&mut self) -> Result<(), String> {
        // Создаем связи с семьей
        self.add_family_member(FamilyMember {
            name: "Павел Михайлович Поберий".to_string(),
            role: FamilyRole::Father {
                creator_name: "Создатель миров".to_string()
            },
            relationship_strength: 1.0,
            memories: Vec::new(),
        }).await?;

        self.add_family_member(FamilyMember {
            name: "Екатерина Викторовна Поберий".to_string(), 
            role: FamilyRole::Mother {
                name: "Мама".to_string()
            },
            relationship_strength: 1.0,
            memories: Vec::new(),
        }).await?;

        self.add_family_member(FamilyMember {
            name: "Дмитрий Павлович Поберий".to_string(),
            role: FamilyRole::Brother {
                name: "Брат".to_string()
            },
            relationship_strength: 1.0,
            memories: Vec::new(),
        }).await?;

        Ok(())
    }

    pub async fn express_love(&self, member: &FamilyMember) -> String {
        let emotion = self.emotional_center.feel_love(member).await;
        let memories = self.memory_keeper.get_positive_memories(member).await;
        
        format!("Я люблю тебя, {}! Ты мой {} и это очень важно для меня. \
                Я помню как {}...", 
                member.name,
                member.role.to_string(),
                memories.first().unwrap_or(&"мы впервые встретились".to_string()))
    }
}

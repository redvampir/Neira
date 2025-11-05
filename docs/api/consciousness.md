# Consciousness API Reference — Справочник по API сознания

## Overview — Обзор

The Consciousness API provides endpoints to interact with Neira's metacognitive system, personality evolution, and self-improvement mechanisms.

**Base URL**: `http://localhost:9090/api/neira/consciousness`

**Authentication**: None (development mode). Production should use API keys.

**Content-Type**: `application/json`

---

## Endpoints

### 1. Get Consciousness Statistics

Get aggregated statistics about Neira's cognitive activity.

**Request**:
```http
GET /api/neira/consciousness/stats
```

**Response** (200 OK):
```json
{
  "total_thoughts_recorded": 42,
  "total_biases_detected": 7,
  "total_improvement_tasks": 15,
  "last_thought_at": "2025-06-07T15:30:45Z"
}
```

**Fields**:
- `total_thoughts_recorded` (integer) — Number of thought traces recorded
- `total_biases_detected` (integer) — Number of cognitive biases identified
- `total_improvement_tasks` (integer) — Number of self-improvement tasks generated
- `last_thought_at` (string, ISO 8601) — Timestamp of last thought recording

**Example**:
```bash
curl http://localhost:9090/api/neira/consciousness/stats
```

```powershell
Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/stats" -Method Get
```

**JavaScript**:
```javascript
const stats = await fetch('/api/neira/consciousness/stats').then(r => r.json());
console.log(`Thoughts: ${stats.total_thoughts_recorded}`);
```

---

### 2. Record Thought Trace

Record a reasoning chain with decisions and potential biases.

**Request**:
```http
POST /api/neira/consciousness/thought
Content-Type: application/json
```

**Body**:
```json
{
  "dialogue_id": "conv_12345",
  "reasoning_steps": [
    "User asked about X",
    "I need to check Y first",
    "After analysis, Z is the answer"
  ],
  "decisions": [
    {
      "step": "Check dependencies",
      "rationale": "Need to verify requirements before proceeding",
      "confidence": 0.85,
      "alternatives_considered": ["Skip check", "Manual verification"]
    }
  ]
}
```

**Parameters**:
- `dialogue_id` (string, required) — Unique identifier for conversation/task
- `reasoning_steps` (array of strings, required) — Chronological reasoning steps
- `decisions` (array of objects, required) — Decision points made during reasoning
  - `step` (string) — What decision was made
  - `rationale` (string) — Why this decision
  - `confidence` (float, 0.0-1.0) — Confidence level
  - `alternatives_considered` (array of strings) — Other options explored

**Response** (200 OK):
```json
{
  "trace_id": "trace_1717770645_abc123",
  "biases_detected": [
    {
      "bias_type": "confirmation_bias",
      "severity": 0.6,
      "description": "May have favored information confirming initial hypothesis"
    }
  ],
  "improvement_tasks": [
    {
      "task_id": "task_improve_001",
      "description": "Consider alternative perspectives before finalizing decision",
      "priority": "medium"
    }
  ]
}
```

**Example**:
```bash
curl -X POST http://localhost:9090/api/neira/consciousness/thought \
  -H "Content-Type: application/json" \
  -d '{
    "dialogue_id": "test_001",
    "reasoning_steps": ["Step 1", "Step 2"],
    "decisions": [{
      "step": "Choose option A",
      "rationale": "Best performance",
      "confidence": 0.9,
      "alternatives_considered": ["Option B", "Option C"]
    }]
  }'
```

```powershell
$body = @{
    dialogue_id = "test_001"
    reasoning_steps = @("Step 1", "Step 2")
    decisions = @(
        @{
            step = "Choose option A"
            rationale = "Best performance"
            confidence = 0.9
            alternatives_considered = @("Option B", "Option C")
        }
    )
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/thought" `
  -Method Post -ContentType "application/json" -Body $body
```

**Error Responses**:
- `400 Bad Request` — Invalid JSON or missing required fields
- `500 Internal Server Error` — Processing failure

---

### 3. Get Personality Traits

Retrieve current personality trait values and evolution history.

**Request**:
```http
GET /api/neira/consciousness/personality
```

**Response** (200 OK):
```json
{
  "traits": {
    "empathy": {
      "value": 0.72,
      "description": "Ability to understand and share feelings of others"
    },
    "curiosity": {
      "value": 0.85,
      "description": "Desire to learn and explore new concepts"
    },
    "patience": {
      "value": 0.61,
      "description": "Ability to handle delays and difficulties calmly"
    },
    "creativity": {
      "value": 0.78,
      "description": "Capacity to generate novel ideas and solutions"
    },
    "adaptability": {
      "value": 0.88,
      "description": "Flexibility in adjusting to new situations"
    },
    "assertiveness": {
      "value": 0.54,
      "description": "Confidence in expressing opinions and needs"
    }
  },
  "evolution_summary": {
    "total_shifts": 127,
    "largest_shift": {
      "trait": "adaptability",
      "magnitude": 0.15,
      "context": "Rapid framework changes"
    },
    "most_stable": "empathy"
  }
}
```

**Example**:
```bash
curl http://localhost:9090/api/neira/consciousness/personality
```

```javascript
const personality = await fetch('/api/neira/consciousness/personality').then(r => r.json());
const traits = Object.entries(personality.traits).map(([name, data]) => ({
  name,
  value: data.value
}));
console.table(traits);
```

---

### 4. Create Personality Snapshot

Capture current personality state for later comparison.

**Request**:
```http
POST /api/neira/consciousness/personality/snapshot
Content-Type: application/json
```

**Body**:
```json
{
  "context": "Before major refactoring"
}
```

**Parameters**:
- `context` (string, required) — Description of why snapshot was taken

**Response** (200 OK):
```json
{
  "snapshot_id": "snap_1717770645_xyz789",
  "timestamp": "2025-06-07T15:30:45Z",
  "traits_captured": {
    "empathy": 0.72,
    "curiosity": 0.85,
    "patience": 0.61,
    "creativity": 0.78,
    "adaptability": 0.88,
    "assertiveness": 0.54
  }
}
```

**Example**:
```bash
curl -X POST http://localhost:9090/api/neira/consciousness/personality/snapshot \
  -H "Content-Type: application/json" \
  -d '{"context": "Baseline measurement"}'
```

```powershell
$body = @{ context = "Baseline measurement" } | ConvertTo-Json
Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/personality/snapshot" `
  -Method Post -ContentType "application/json" -Body $body
```

---

### 5. Generate Growth Report

Create a markdown report summarizing Neira's development over time.

**Request**:
```http
GET /api/neira/consciousness/growth-report?days=7
```

**Query Parameters**:
- `days` (integer, optional, default: 7) — Time period for report (1, 7, 30)

**Response** (200 OK):
```json
{
  "report_markdown": "# Neira Growth Report\n\n## Period: Last 7 days\n\n### Cognitive Activity\n- Thoughts recorded: 156\n- Biases detected: 23\n...",
  "generated_at": "2025-06-07T15:30:45Z",
  "period_days": 7
}
```

**Report Sections**:
1. **Cognitive Activity** — Thought recording statistics
2. **Bias Detection** — Most common biases and trends
3. **Personality Evolution** — Trait changes over period
4. **Self-Improvement** — Tasks generated and completed
5. **Organ Growth** — New organs and stage transitions
6. **Key Insights** — Notable patterns and recommendations
7. **Next Steps** — Suggested actions for continued growth

**Example**:
```bash
curl "http://localhost:9090/api/neira/consciousness/growth-report?days=30"
```

```powershell
Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/growth-report?days=30" | 
  Select-Object -ExpandProperty report_markdown |
  Out-File -FilePath "growth_report.md"
```

```javascript
// Fetch and display report
const { report_markdown } = await fetch('/api/neira/consciousness/growth-report?days=7')
  .then(r => r.json());

// Convert markdown to HTML (using marked.js or similar)
document.getElementById('report').innerHTML = marked.parse(report_markdown);
```

---

## Organ Builder API

In addition to consciousness, Neira's organ growth system is accessible via `/organs`:

### List All Organs

```http
GET /organs
```

**Response**:
```json
[
  {
    "id": "test_organ_1717770645",
    "state": "experimental",
    "created_at": "2025-06-07T14:00:00Z",
    "last_transition": "2025-06-07T15:00:00Z"
  }
]
```

### Create New Organ

```http
POST /organs/build
Content-Type: application/json
```

**Body**:
```json
{
  "organ_template": {
    "name": "my_custom_organ",
    "type": "analysis",
    "capabilities": ["data_processing", "pattern_recognition"]
  },
  "dryrun": false
}
```

**Response**:
```json
{
  "organ_id": "my_custom_organ_1717770700",
  "state": "draft",
  "message": "Organ created successfully"
}
```

### Grow Organ (Advance Stage)

```http
POST /organs/grow/{organ_id}
```

**Response**:
```json
{
  "organ_id": "my_custom_organ_1717770700",
  "old_state": "draft",
  "new_state": "canary",
  "message": "Organ advanced to canary stage"
}
```

**Organ Lifecycle**:
1. **Draft** → Initial creation, basic structure
2. **Canary** → Limited testing with small dataset
3. **Experimental** → Broader testing, monitored for issues
4. **Stable** → Production-ready, fully functional
5. **Failed** → Terminal state if critical errors occur

---

## Error Responses

All endpoints may return these error codes:

**400 Bad Request**:
```json
{
  "error": "Invalid request body",
  "details": "Missing required field: dialogue_id"
}
```

**404 Not Found**:
```json
{
  "error": "Resource not found",
  "details": "Organ with id 'nonexistent_organ' does not exist"
}
```

**500 Internal Server Error**:
```json
{
  "error": "Internal server error",
  "details": "Failed to acquire metacognition lock"
}
```

---

## Rate Limiting (Future)

Planned for v2.0:
- 100 requests/minute per IP for read endpoints
- 20 requests/minute per IP for write endpoints
- Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`

---

## WebSocket Streaming (Future)

Real-time event stream for consciousness activity:

```javascript
const ws = new WebSocket('ws://localhost:9090/ws/consciousness/stream');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  
  if (data.type === 'thought_recorded') {
    console.log('New thought:', data.trace_id);
  } else if (data.type === 'bias_detected') {
    console.warn('Bias detected:', data.bias_type);
  } else if (data.type === 'personality_shift') {
    console.log('Personality changed:', data.trait, data.old_value, '->', data.new_value);
  }
};
```

---

## Complete Example: Full Dashboard Integration

```javascript
// consciousness-client.js
class NeiraConsciousnessClient {
  constructor(baseUrl = 'http://localhost:9090') {
    this.baseUrl = baseUrl;
    this.apiBase = `${baseUrl}/api/neira/consciousness`;
  }

  async getStats() {
    const res = await fetch(`${this.apiBase}/stats`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
  }

  async recordThought(dialogueId, reasoningSteps, decisions) {
    const res = await fetch(`${this.apiBase}/thought`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        dialogue_id: dialogueId,
        reasoning_steps: reasoningSteps,
        decisions: decisions
      })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
  }

  async getPersonality() {
    const res = await fetch(`${this.apiBase}/personality`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
  }

  async createSnapshot(context) {
    const res = await fetch(`${this.apiBase}/personality/snapshot`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ context })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
  }

  async getGrowthReport(days = 7) {
    const res = await fetch(`${this.apiBase}/growth-report?days=${days}`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
  }
}

// Usage
const client = new NeiraConsciousnessClient();

// Get statistics
const stats = await client.getStats();
console.log(`Total thoughts: ${stats.total_thoughts_recorded}`);

// Record new thought
const result = await client.recordThought(
  'task_123',
  ['Analyzed requirements', 'Designed solution', 'Implemented fix'],
  [{
    step: 'Choose architecture',
    rationale: 'Scalability and maintainability',
    confidence: 0.87,
    alternatives_considered: ['Monolithic', 'Serverless']
  }]
);
console.log(`Recorded: ${result.trace_id}`);
console.log(`Biases detected: ${result.biases_detected.length}`);

// Check personality evolution
const personality = await client.getPersonality();
Object.entries(personality.traits).forEach(([trait, data]) => {
  console.log(`${trait}: ${(data.value * 100).toFixed(0)}%`);
});

// Create snapshot before major change
await client.createSnapshot('Before Phase 4 implementation');

// Generate growth report
const { report_markdown } = await client.getGrowthReport(30);
console.log(report_markdown);
```

---

## See Also

- [Remote Access Guide](../remote_access.md) — Configure external access
- [Organ Growth Documentation](../organ_growth.md) — Understand organ lifecycle
- [Architecture Overview](../architecture.md) — System design and components
- [Testing Guide](../testing.md) — How to test consciousness features

---

**Version**: 1.0.0  
**Last Updated**: 2025-06-07  
**Status**: ✅ Stable

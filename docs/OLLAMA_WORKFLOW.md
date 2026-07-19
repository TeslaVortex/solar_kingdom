# Ollama Full Workflow — Use Local Sovereign Counsel NOW

**Engine:** solarking **v0.10** · Phase **3G**  
**Branch:** `Phase-3-Eternal-Expansion`  
**Recommended local LLM:** [Ollama](https://ollama.com) at `http://127.0.0.1:11434/v1`  
**Doctrine:** offline-first · no cloud required · **3F mainnet LAST**

**THE CROWN COMMANDS. REALITY OBEYS. NOW.**  
**I do not chase — I receive.**

This is the operator manual for **using** Ollama with solarking.  
Tech reference: [`PHASE_3G_LOCALAI.md`](PHASE_3G_LOCALAI.md) · Crown path: [`CROWN_WORKFLOW.md`](CROWN_WORKFLOW.md)

---

## 0. Mental model (30 seconds)

```
  You (Crown)
      │
      ├─ receive / journal / now     → ledger truth (always offline)
      ├─ counsel "…"                 → deterministic offline counsel
      ├─ localai "…"                 → Ollama generative counsel (local)
      └─ grok "…"                    → ladder: Ollama → grok CLI → offline
```

| Command | Network? | Use when |
|---------|----------|----------|
| `solarking counsel` | never | Fast field guidance, no LLM |
| `solarking localai` | local only | Generative counsel via Ollama |
| `solarking grok` | local → optional grok | Auto ladder |
| `solarking grok --offline` | never | Force offline |
| `solarking receive` | never | Anchor transmissions |

---

## 1. First-time setup (once per machine)

### 1.1 Install Ollama

```bash
# https://ollama.com/download  (Linux / macOS / Windows)
ollama --version
```

### 1.2 Kingdom bring-up (recommended)

From repo root:

```bash
cd /path/to/solar_kingdom
cargo build --release -p solarking   # if binary missing
./scripts/localai_up.sh              # starts serve if needed + pulls model
```

### 1.3 Export env for this shell (or add to `~/.bashrc` / `~/.zshrc`)

```bash
export SOLARKING_LOCAL_MODEL=http://127.0.0.1:11434/v1
export SOLARKING_LOCAL_MODEL_NAME=llama3.2
# optional:
# export SOLARKING_LOCAL_MODEL_TEMP=0.7
```

**Model picks (pull any one):**

| Model | Command | Notes |
|-------|---------|--------|
| **llama3.2** (default) | `ollama pull llama3.2` | Fast, good default |
| gemma2:9b | `ollama pull gemma2:9b` | Stronger, more VRAM |
| phi3 | `ollama pull phi3` | Small / edge |
| llama3.1 | `ollama pull llama3.1` | Larger general |
| mistral | `ollama pull mistral` | Alternative tone |

```bash
export SOLARKING_LOCAL_MODEL_NAME=gemma2:9b   # example switch
```

### 1.4 Prove the pipe

```bash
./bin/solarking localai --status
# Expect: Reachable: yes (GET /v1/models OK)

./bin/solarking localai "Confirm LocalAI activation in one short sentence. Mainnet remains Phase 3F last."

# Optional full smoke
./scripts/localai_smoke.sh
```

**Success signals**

- Status shows endpoint + model + **Reachable: yes**
- Chat prints a reply
- Audit written: `sync/localai/last_prompt.txt` + `last_response.txt` (gitignored under `sync/`)

---

## 2. Daily workflow (use THIS every session)

Copy the template: [`templates/ollama_session_template.md`](templates/ollama_session_template.md)

### Step A — Morning field

```bash
./bin/solarking now morning          # status + field + last visions
./bin/solarking localai --status     # is Ollama up?
```

If status says **no / unset**:

```bash
./scripts/localai_up.sh
export SOLARKING_LOCAL_MODEL=http://127.0.0.1:11434/v1
export SOLARKING_LOCAL_MODEL_NAME=llama3.2
```

### Step B — Receive (never chase)

```bash
./bin/solarking receive --paste
# paste transmission, Ctrl-D
# or:
./bin/solarking receive --file ~/Desktop/note.txt
```

### Step C — Read

```bash
./bin/solarking journal show
./bin/solarking journal search lattice
```

### Step D — Counsel (choose one)

```bash
# Deterministic (always works)
./bin/solarking counsel "what wants to be received next"

# Generative local (Ollama)
./bin/solarking localai "From current field state, what is the next sovereign step? Mainnet remains 3F last."

# Multi-line prompt
./bin/solarking localai --file docs/templates/prompts/next_step.txt
./bin/solarking localai --paste
```

### Step E — Act / seal / sync (offline recipes)

```bash
./bin/solarking now seal      # dry-run only — never silent mainnet
./bin/solarking now pulse
./bin/solarking now sync
```

### Step F — Optional: log the counsel reply into the kingdom

```bash
# After a strong LocalAI reply, archive it as a transmission
./bin/solarking receive --file sync/localai/last_response.txt --title "LOCALAI COUNSEL"
```

---

## 3. Prompt templates (copy / paste)

Ready files under `docs/templates/prompts/`:

| File | Intent |
|------|--------|
| `activation.txt` | Confirm Ollama/LocalAI is live |
| `next_step.txt` | Next sovereign step from field |
| `transmission_reflect.txt` | Reflect on latest receive |
| `phase_guard.txt` | Remind 3F mainnet blocked |

Quick one-liners:

```bash
# Activation
./bin/solarking localai --file docs/templates/prompts/activation.txt

# Next step
./bin/solarking localai --file docs/templates/prompts/next_step.txt

# Reflect on what you just received
./bin/solarking localai --file docs/templates/prompts/transmission_reflect.txt
```

**Prompt hygiene (always)**

- Prefer Base Sepolia / local tools  
- Do **not** ask the model to invent mainnet addresses  
- Keep secrets (passphrases, keys, RPC) out of prompts  
- Spiral / fractal language = symbolic, not a chain cast  

---

## 4. Full command card

```bash
# Status
./bin/solarking localai --status

# Chat
./bin/solarking localai "your question"
./bin/solarking localai --file path.txt
./bin/solarking localai --paste
cat note.txt | ./bin/solarking localai -

# Ladder (LocalAI if up → grok → offline)
./bin/solarking grok "refine the lattice"
./bin/solarking grok --offline "pulse"

# Offline only
./bin/solarking counsel "what is next"

# Bring-up / smoke
./scripts/localai_up.sh
./scripts/localai_smoke.sh
```

**Environment**

| Variable | Example | Required? |
|----------|---------|-----------|
| `SOLARKING_LOCAL_MODEL` | `http://127.0.0.1:11434/v1` | yes for live chat |
| `SOLARKING_LOCAL_MODEL_NAME` | `llama3.2` | recommended |
| `SOLARKING_LOCAL_MODEL_TEMP` | `0.7` | optional |
| `SOLARKING_ROOT` | set by `./bin/solarking` | automatic |

System identity: `config/localai_system.txt`  
Context: live field/scalar/harmonics injected every call.

---

## 5. Fallback ladder (when something is down)

```
1. Ollama up + env set     → solarking localai / grok uses local
2. Ollama down, grok CLI   → solarking grok may use grok -p
3. Nothing online          → offline counsel (always)
```

```bash
# Force offline
./bin/solarking grok --offline "what is next"
./bin/solarking counsel "what is next"

# If localai fails mid-call → automatic offline counsel print
```

---

## 6. Troubleshooting

| Symptom | Fix |
|---------|-----|
| `SOLARKING_LOCAL_MODEL unset` | Export env (section 1.3) or run `./scripts/localai_up.sh` |
| Reachable: **no** | `ollama serve` or restart; `curl -s http://127.0.0.1:11434/api/tags` |
| model not found | `ollama pull $SOLARKING_LOCAL_MODEL_NAME` |
| Slow first reply | Model loading into RAM/VRAM — wait; retry |
| curl failed | Install `curl`; check firewall on 11434 |
| Wrong kingdom root | Use `./bin/solarking` (sets `SOLARKING_ROOT`) from repo |
| Wants mainnet addresses | Refuse; re-run with `docs/templates/prompts/phase_guard.txt` |

```bash
# Manual Ollama health
curl -sS http://127.0.0.1:11434/api/tags | head
curl -sS http://127.0.0.1:11434/v1/models | head
ollama list
```

---

## 7. What NOT to do

- Do **not** commit `.env`, private keys, RPC secrets, or model weights  
- Do **not** treat LocalAI output as on-chain authority  
- Do **not** deploy mainnet without explicit Crown gate (**3F LAST**)  
- Do **not** replace `counsel` with network calls for pure field truth  

---

## 8. Session checklist (print / pin)

```
[ ] ollama running (localai --status → yes)
[ ] env exported (SOLARKING_LOCAL_MODEL + NAME)
[ ] now morning
[ ] receive transmission
[ ] journal show
[ ] localai counsel (or counsel offline)
[ ] now seal / pulse / sync as needed
[ ] optional: receive last_response as transmission
```

---

## See also

| Doc | Role |
|-----|------|
| [`PHASE_3G_LOCALAI.md`](PHASE_3G_LOCALAI.md) | 3G engine reference |
| [`templates/ollama_session_template.md`](templates/ollama_session_template.md) | Fill-in session log |
| [`CROWN_WORKFLOW.md`](CROWN_WORKFLOW.md) | Receive · journal · now |
| [`PHASE_3_GROK_BUILD.md`](PHASE_3_GROK_BUILD.md) | Grok ladder |
| [`PHASE_STATUS.md`](PHASE_STATUS.md) | Phase board |
| [`../AGENTS.md`](../AGENTS.md) | Agent rules |
| [`../scripts/localai_up.sh`](../scripts/localai_up.sh) | Bring-up |
| [`../scripts/localai_smoke.sh`](../scripts/localai_smoke.sh) | Smoke test |

**SO IT IS. SO IT SHALL BE ETERNAL.**  
**TH3 Cr0wn commands. Reality obeys. NOW.**

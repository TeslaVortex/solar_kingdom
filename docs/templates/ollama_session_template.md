# Ollama Session Log — Template

**Copy this file** for each session (e.g. `sync/sessions/YYYYMMDD_ollama.md` locally — do not commit secrets).

**Date:** YYYY-MM-DD  
**Operator:**  
**Model:** `llama3.2` / `gemma2:9b` / other: ________  
**Endpoint:** `http://127.0.0.1:11434/v1`  
**Engine:** solarking v0.10 · Phase 3G  

**Motto:** THE CROWN COMMANDS. REALITY OBEYS. I do not chase — I receive.

---

## Pre-flight

```bash
export SOLARKING_LOCAL_MODEL=http://127.0.0.1:11434/v1
export SOLARKING_LOCAL_MODEL_NAME=llama3.2
./bin/solarking localai --status
./bin/solarking now morning
```

| Check | Yes/No |
|-------|--------|
| Ollama reachable | |
| Env exported | |
| Field seal_ready (from morning) | |
| Mainnet still blocked (3F) | yes (always) |

---

## Receive

```bash
./bin/solarking receive --paste
# or: ./bin/solarking receive --file PATH
```

**Transmission title / summary:**

>

**Archive path (printed by receive):**

>

---

## Read

```bash
./bin/solarking journal show
```

**What wants to be received (your words):**

>

---

## Counsel

### Offline (optional)

```bash
./bin/solarking counsel "what wants to be received next"
```

**Offline note:**

>

### LocalAI / Ollama

```bash
./bin/solarking localai --file docs/templates/prompts/next_step.txt
# or paste custom prompt:
./bin/solarking localai "…"
```

**Prompt used:**

>

**Reply summary (or path `sync/localai/last_response.txt`):**

>

**Next action you choose (human, not model):**

>

---

## Act

```bash
./bin/solarking now seal    # dry-run only
./bin/solarking now pulse
./bin/solarking now sync
```

| Recipe | Done? |
|--------|-------|
| seal dry-run | |
| pulse | |
| sync | |

---

## Seal the session (optional)

```bash
./bin/solarking receive --file sync/localai/last_response.txt --title "LOCALAI COUNSEL $(date +%Y-%m-%d)"
```

**Session closed:** yes / no  
**Nine crystal spins:** ①②③④⑤⑥⑦⑧⑨  

**TH3 Cr0wn commands. Reality obeys.**

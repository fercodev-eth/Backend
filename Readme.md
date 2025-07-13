# Documentación

## Descripción General
DonatifiContract es un contrato inteligente desarrollado en Rust para la blockchain Soroban que permite crear y gestionar eventos de donación descentralizados. El contrato facilita la creación de campañas de fundraising donde los usuarios pueden donar tokens específicos para alcanzar metas establecidas.

## Características Principales

- ✅ Creación de eventos de donación con metas específicas
- ✅ Gestión de estados de eventos (Abierto/Cerrado)
- ✅ Sistema de fechas límite para donaciones
- ✅ Soporte para múltiples tipos de tokens
- ✅ Autenticación requerida para operaciones críticas

## Arquitectura del Contrato

### Dependencias

```rust
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec, contracttype};
```

El contrato utiliza el SDK de Soroban con las siguientes funcionalidades:
- `contract` y `contractimpl`: Macros para definir contratos
- `Address`: Manejo de direcciones de usuarios
- `Env`: Entorno de ejecución del contrato
- `Symbol` y `Vec`: Tipos de datos para almacenamiento
- `contracttype`: Macro para tipos de datos serializables

### Tipos de Datos

#### Enum Status
```rust
#[derive(Clone, Copy)]
#[contracttype]
pub enum Status {
    Open,    // Evento abierto para donaciones
    Closed,  // Evento cerrado (meta alcanzada o expirado)
}
```

#### Struct DonationEvent
```rust
#[contracttype]
#[derive(Clone)]
pub struct DonationEvent {
    pub owner: Address,           // Propietario del evento
    pub token_reward: Address,    // Token que se otorga como recompensa
    pub token_donation: Address,  // Token aceptado para donaciones
    pub amount_raised: i128,      // Cantidad recaudada actual
    pub goal_amount: i128,        // Meta de recaudación
    pub deadline: u64,            // Fecha límite (timestamp)
    pub status: Status,           // Estado del evento
}
```

### Almacenamiento

El contrato utiliza un símbolo constante para almacenar eventos:
```rust
const EVENTS: Symbol = symbol_short!("EVENTS");
```

Los eventos se almacenan como un vector en el almacenamiento de instancia del contrato.

## Funciones del Contrato

### 1. `create_evt` - Crear Evento

```rust
pub fn create_evt(
    env: Env,
    owner: Address,
    token_reward: Address,
    token_donation: Address,
    goal_amount: i128,
    deadline: u64,
) -> u32
```

**Descripción**: Crea un nuevo evento de donación.

**Parámetros**:
- `env`: Entorno de ejecución
- `owner`: Dirección del propietario del evento
- `token_reward`: Dirección del token de recompensa
- `token_donation`: Dirección del token aceptado para donaciones
- `goal_amount`: Meta de recaudación
- `deadline`: Fecha límite para donaciones

**Retorna**: ID del evento creado (índice en el vector)

**Características**:
- ✅ Requiere autenticación del propietario
- ✅ Inicializa `amount_raised` en 0
- ✅ Establece estado inicial como `Open`
- ✅ Almacena el evento en el storage

### 2. `get_event` - Obtener Evento

```rust
pub fn get_event(env: Env, id: u32) -> Option<DonationEvent>
```

**Descripción**: Recupera un evento específico por su ID.

**Parámetros**:
- `env`: Entorno de ejecución
- `id`: ID del evento a recuperar

**Retorna**: `Option<DonationEvent>` - El evento si existe, `None` si no existe

**Características**:
- ✅ Función de solo lectura
- ✅ Manejo seguro de índices inválidos

### 3. `list_events` - Listar Eventos

```rust
pub fn list_events(env: Env) -> Vec<DonationEvent>
```

**Descripción**: Retorna todos los eventos almacenados.

**Parámetros**:
- `env`: Entorno de ejecución

**Retorna**: Vector con todos los eventos

**Características**:
- ✅ Función de solo lectura
- ✅ Retorna vector vacío si no hay eventos

### 4. `donate` - Realizar Donación

```rust
pub fn donate(env: Env, donor: Address, id: u32, amount: i128)
```

**Descripción**: Permite a un usuario donar a un evento específico.

**Parámetros**:
- `env`: Entorno de ejecución
- `donor`: Dirección del donante
- `id`: ID del evento
- `amount`: Cantidad a donar

**Características**:
- ✅ Requiere autenticación del donante
- ✅ Valida que el evento esté abierto
- ✅ Verifica que no haya expirado la fecha límite
- ✅ Actualiza el monto recaudado
- ✅ Cierra automáticamente el evento si se alcanza la meta

**Validaciones**:
- El evento debe existir
- El evento debe estar en estado `Open`
- La fecha actual debe ser menor o igual al deadline
- Si `amount_raised >= goal_amount`, el evento se cierra automáticamente

## Flujo de Uso

### 1. Creación de Evento
```
Usuario → create_evt() → Evento creado con ID único
```

### 2. Consulta de Eventos
```
Usuario → get_event(id) → Detalles del evento
Usuario → list_events() → Lista completa de eventos
```

### 3. Proceso de Donación
```
Donante → donate(donor, id, amount) → Validaciones → Actualización del evento
```

## Consideraciones de Seguridad

### Autenticación
- `create_evt`: Requiere autenticación del propietario
- `donate`: Requiere autenticación del donante

### Validaciones
- Verificación de estado del evento (debe estar abierto)
- Validación de fecha límite
- Manejo de errores con `expect()` y `assert!()`

### Almacenamiento
- Uso de almacenamiento de instancia para persistencia
- Inicialización segura con `unwrap_or(vec![&env])`

## Limitaciones Identificadas

1. **Transferencia de Tokens**: El contrato no maneja transferencias reales de tokens
2. **Distribución de Recompensas**: No implementa lógica de distribución de recompensas
3. **Validación de Tokens**: No valida que los tokens especificados existan
4. **Límites de Gas**: No considera optimizaciones de gas para vectores grandes
5. **Permisos**: No implementa roles o permisos granulares

## Mejoras Sugeridas

1. Implementar transferencias reales de tokens usando estándares como SEP-41
2. Agregar sistema de recompensas automáticas
3. Implementar validación de contratos de tokens
4. Añadir eventos de logging para auditoría
5. Implementar sistema de roles (admin, moderador, etc.)
6. Agregar funcionalidad de cancelación de eventos
7. Implementar reembolsos automáticos para eventos fallidos

## Ejemplo de Uso

```rust
// Crear evento
let event_id = DonatifiContract::create_evt(
    env,
    owner_address,
    reward_token_address,
    donation_token_address,
    1000000, // Meta: 1,000,000 tokens
    1234567890 // Deadline timestamp
);

// Donar al evento
DonatifiContract::donate(
    env,
    donor_address,
    event_id,
    50000 // Donación: 50,000 tokens
);

// Consultar evento
let event = DonatifiContract::get_event(env, event_id);
```

## Conclusión

DonatifiContract proporciona una base sólida para un sistema de donaciones descentralizado, implementando las funcionalidades core necesarias con validaciones de seguridad apropiadas. 
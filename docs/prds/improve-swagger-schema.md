# PRD: FRED API OpenAPI Schema Enhancement

## Problem Statement

The current OpenAPI specification for the FRED API contains generic response
schemas (`type: object` or `type: string`) that prevent the Rust code generator
from creating proper data structures and typed responses. This results in
generated functions that return `()` instead of actual parsed data.

## Objective

Enhance the OpenAPI specification with detailed response schemas that
accurately represent FRED API JSON responses, enabling the generation of
strongly-typed Rust client code.

## Scope

- **In Scope**: All FRED API endpoints in the current specification
- **Out of Scope**: XML response support (JSON only)
- **Focus**: Response schema definitions and data structure modeling

## Requirements

### 1. Response Schema Analysis

- Examine actual FRED API JSON responses for each endpoint
- Document the structure, field types, and nested objects
- Identify common patterns and reusable components

### 2. Schema Definition

- Create detailed OpenAPI schemas in `components/schemas` section
- Define proper data types (integer, string, array, object, date, etc.)
- Use consistent naming conventions (e.g., PascalCase for schema names)
- Include required/optional field specifications

### 3. Schema Implementation

Replace generic response schemas like:

```yaml
application/json:
  schema:
    type: object
```

With specific schema references:

```yaml
application/json:
  schema:
    $ref: "#/components/schemas/CategoryResponse"
```

### 4. Common Schema Patterns

Define reusable schemas for:

- Error responses
- Pagination metadata
- Date/time fields
- FRED-specific data structures (categories, series, observations, etc.)

### 5. Validation

- Ensure all endpoints have proper response schemas
- Remove XML content types (JSON-only approach)
- Validate schema accuracy against actual API responses

## Expected Outcome

- Generated Rust functions return proper data types instead of `()`
- Type-safe client code with deserialization
- Improved developer experience with IntelliSense/autocomplete
- Reduced runtime errors through compile-time type checking

## Success Criteria

- All endpoints generate functions with meaningful return types
- Generated code compiles without schema-related errors
- Response parsing works correctly with actual FRED API data


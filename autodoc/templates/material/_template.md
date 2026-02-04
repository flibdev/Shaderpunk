# Material: {{ name }}

{{desc}}

## Techniques

| Index | Pass | Vertex Factory | Supported Flags |
|-------|------|----------------|-----------------|
{{#each techniques}}{{#with desc}}| {{index}} | {{pass}} | {{>link vertex_factory}} | {{#if is_discarded}}`DC` {{/if}}{{#if is_dismembered}}`DM` {{/if}}{{#if is_preskinned}}`PS` {{/if}}|{{/with}}
{{/each}}



## Vertex Factories
<details>
<summary>Supports {{len vfs}} Vertex Factories</summary>

{{#each vfs}}
- {{>link}}
{{/each}}
</details>

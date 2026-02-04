

#### Samplers

| Shader | Register | Filter Min | Filter Mag | Filter Mip | Tex U | Tex V | Tex W | Comp Func | 
|--------|----------|------------|------------|------------|-------|-------|-------|-----------|
{{#each vs_samplers}}
| VS {{@index}} | {{register}} | {{filteringMin}} | {{filteringMag}} | {{filteringMip}} | {{addressU}} | {{addressV}} | {{addressW}} | {{comparisonFunc}} |
{{/each}}
{{#each ps_samplers}}
| PS {{@index}} | {{register}} | {{filteringMin}} | {{filteringMag}} | {{filteringMip}} | {{addressU}} | {{addressV}} | {{addressW}} | {{comparisonFunc}} |
{{/each}}
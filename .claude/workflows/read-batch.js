export const meta = {
  name: 'read-batch',
  description: 'Read a batch of drawings: one fresh reader per filled-in brief, in parallel, and one compact result for the batch',
  whenToUse: 'A batch of reference drawings is to be read, and each reader has a filled-in brief from docs/agents/reader-brief.md in its own folder. Its model has been chosen with Jon.',
  phases: [
    { title: 'Read', detail: 'one fresh agent per brief, each told only to read its brief and do what it says' },
  ],
}

// How it is called, and what a brief is: docs/agents/reader-brief.md.
//
// args: {
//   model: the model Jon approved for the readers, as the Workflow tool names it. Required:
//          a reader's model is chosen with Jon before launch.
//   briefs: the whole paths of the filled-in briefs, one file per reader per work, each inside
//           that reader's own folder.
// }
//
// It returns one entry per brief, in the order given: the readings, whether each passed its
// check, the view and level judged, the head's height, whether any command was refused, and at
// most three lines on where the instructions failed. A reader's whole report stays in its
// folder, where its brief tells it to write it.

const READING = {
  type: 'object',
  properties: {
    path: { type: 'string', description: 'The reading file, as the brief names it' },
    passed_check: { type: 'boolean', description: 'Whether check-reading last said the reading is complete' },
    view: { type: 'string', enum: ['front', 'three-quarter', 'side', 'set aside'] },
    level: { type: 'string', enum: ['level', 'not level', 'set aside'] },
    head_height_px: { type: ['number', 'null'], description: 'The head\'s height in pixels of the picture, as the report gives it; null when set aside' },
  },
  required: ['path', 'passed_check', 'view', 'level', 'head_height_px'],
}

const RESULT = {
  type: 'object',
  properties: {
    readings: { type: 'array', items: READING },
    refused: {
      type: 'object',
      properties: {
        any: { type: 'boolean', description: 'Whether the permission system refused any command' },
        command: { type: 'string', maxLength: 300, description: 'The first command refused, or empty' },
      },
      required: ['any', 'command'],
    },
    unclear: {
      type: 'array',
      maxItems: 3,
      items: { type: 'string', maxLength: 240 },
      description: 'At most three lines on where the reading instructions were unclear or failed',
    },
    report: { type: 'string', description: 'The path of the whole report the brief asked for' },
  },
  required: ['readings', 'refused', 'unclear', 'report'],
}

const given = args || {}
if (typeof given.model !== 'string' || given.model === '') {
  throw new Error('args.model is required: a reader\'s model is chosen with Jon before launch')
}
if (!Array.isArray(given.briefs) || given.briefs.length === 0) {
  throw new Error('args.briefs is required: the whole paths of the filled-in briefs')
}
for (const brief of given.briefs) {
  if (typeof brief !== 'string' || !brief.startsWith('/')) {
    throw new Error(`a brief is named by its whole path: ${JSON.stringify(brief)}`)
  }
}
if (new Set(given.briefs).size !== given.briefs.length) {
  throw new Error('a brief is named twice: each reader reads once, from its own brief')
}

phase('Read')
log(`${given.briefs.length} briefs, one fresh reader each, on ${given.model}`)

const results = await parallel(given.briefs.map((brief, index) => () =>
  agent(
    `Read the brief at ${brief} and do what it says. When you are done, return the result asked for, taken from the report the brief tells you to write.`,
    { label: `reader ${index + 1}`, phase: 'Read', model: given.model, schema: RESULT },
  )))

const batch = given.briefs.map((brief, index) => {
  const result = results[index]
  if (!result) {
    return { brief, ended: 'the reader stopped without a result' }
  }
  return { brief, ...result }
})

const refused = batch.filter((entry) => entry.refused && entry.refused.any).length
const unchecked = batch.filter((entry) =>
  !entry.readings || entry.readings.length === 0 || entry.readings.some((reading) => !reading.passed_check)).length
log(`${batch.length} briefs: ${refused} with a refused command, ${unchecked} without every reading passing its check`)

return batch

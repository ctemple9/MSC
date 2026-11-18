export type InitiationMilestone = { id: string; label: string; done: boolean };

export function initiationMilestones(
  serverType: string,
  flavor: string,
  extensions: readonly string[],
): InitiationMilestone[] {
  const modded = ['fabric', 'forge', 'neoforge', 'quilt'].includes(flavor.toLowerCase());
  const runtime = serverType === 'bedrock' ? 'Bedrock' : flavor || 'Java';
  const startup = [
    ['launch', `Launch ${runtime}`],
    ...(modded ? [['loader', `Begin ${runtime} loader initialization`]] : []),
    ...extensions.map((name, index) => [`extension-${index}`, `Begin loading ${name}`]),
    ['world', 'Begin world preparation'],
    ['ready', 'Receive server-ready confirmation'],
    ['stopped', 'Confirm the server has stopped'],
  ];
  return [
    ...startup.map(([id, label]) => ({ id: `one:${id}`, label: `Pass 1: ${label}`, done: false })),
    { id: 'connections', label: 'Finish connection choices', done: false },
    ...startup.map(([id, label]) => ({ id: `two:${id}`, label: `Pass 2: ${label}`, done: false })),
  ];
}

/** Console hints advance presentation only; engine operations decide readiness and failure. */
export function observedStartupMilestones(
  text: string,
  extensionNames: readonly string[],
): string[] {
  const milestones: string[] = [];
  if (/starting (?:minecraft|bedrock) server|server version:|starting org\./i.test(text))
    milestones.push('launch');
  if (
    /loading minecraft .*fabric|fabric loader|modlauncher running|forge mod loading|neoforge.*loading|quilt loader/i.test(
      text,
    )
  )
    milestones.push('loader');
  if (
    /preparing (?:level|start region)|preparing spawn|spawn area|opening level|generating (?:world|chunks)/i.test(
      text,
    )
  )
    milestones.push('world');
  extensionNames.forEach((name, index) => {
    const named =
      name === 'plugins' || name === 'mods'
        ? /loading \d+ mods|initializing plugins|loading plugins|enabling .+ v/i.test(text)
        : text.toLowerCase().includes(name.toLowerCase()) &&
          /loading|enabling|initializ|started|listening/i.test(text);
    if (named) milestones.push(`extension-${index}`);
  });
  return milestones;
}

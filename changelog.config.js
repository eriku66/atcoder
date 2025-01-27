module.exports = {
  disableEmoji: false,
  format: '{type}{scope}: {emoji}{subject}',
  list: ['solve', 'study', 'refactor', 'add', 'compete', 'fix', 'chore', 'docs', 'perf'],
  maxMessageLength: 64,
  minMessageLength: 3,
  questions: ['type', 'scope', 'subject', 'body'],
  scopes: [],
  types: {
    chore: {
      description: 'Build process or auxiliary tool changes',
      emoji: '🛠️ ',
      value: 'chore'
    },
    docs: {
      description: 'Documentation only changes',
      emoji: '🖊️ ',
      value: 'docs'
    },
    solve: {
      description: 'Solve the problem',
      emoji: '🤩',
      value: 'solve'
    },
    study: {
      description: 'Solve the problems with reference to the explanations',
      emoji: '🧐',
      value: 'study'
    },
    add: {
      description: 'Add problems',
      emoji: '🗃️ ',
      value: 'add'
    },
    compete: {
      description: 'Contest in the contest',
      emoji: '🥳',
      value: 'compete'
    },
    fix: {
      description: 'A bug fix',
      emoji: '👾',
      value: 'fix'
    },
    perf: {
      description: 'A code change that improves performance',
      emoji: '⚡️',
      value: 'perf'
    },
    refactor: {
      description: 'A code change that neither fixes a bug or adds a feature',
      emoji: '💡',
      value: 'refactor'
    },
    messages: {
      type: 'Select the type of change that you\'re committing:',
      customScope: 'Select the scope this component affects:',
      subject: 'Write a short, imperative mood description of the change:\n',
      body: 'Provide a longer description of the change:\n ',
      breaking: 'List any breaking changes:\n',
      footer: 'Issues this commit closes, e.g #123:',
      confirmCommit: 'The packages that this commit has affected\n',
    },
  }
};

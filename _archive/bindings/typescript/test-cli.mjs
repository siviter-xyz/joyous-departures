import { generateGoodbye } from './src/index.ts';

console.log('1. Basic generation:');
console.log(await generateGoodbye());
console.log('');

console.log('2. With custom name:');
console.log(await generateGoodbye({ templateArgs: { name: 'Alice' } }));
console.log('');

console.log('3. Without emojis:');
console.log(await generateGoodbye({ use_emojis: false }));
console.log('');

console.log('4. With custom timezone:');
console.log(await generateGoodbye({ timezone: 'America/New_York' }));
console.log('');

console.log('5. Multiple calls (showing randomness):');
for (let i = 0; i < 3; i++) {
    console.log(`  Call ${i + 1}: ${await generateGoodbye()}`);
}
console.log('');

console.log('✅ TypeScript bindings working!');



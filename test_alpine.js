const fs = require('fs');
const html = fs.readFileSync('static/index.html', 'utf8');

// VERY basic check for unclosed template tags
const openCount = (html.match(/<template/g) || []).length;
const closeCount = (html.match(/<\/template>/g) || []).length;
console.log(`Templates open: ${openCount}, closed: ${closeCount}`);

const openDiv = (html.match(/<div/g) || []).length;
const closeDiv = (html.match(/<\/div>/g) || []).length;
console.log(`Divs open: ${openDiv}, closed: ${closeDiv}`);

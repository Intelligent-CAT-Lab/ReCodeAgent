// This is a patch for 2 fragments in the translated code.
// They are counted as 'user_fix' in evaluation.
function compare(a, b) {
    if (typeof a === 'string' && typeof b === 'string') {
        return a.localeCompare(b);
    } else if (typeof a === 'number' && typeof b === 'number') {
        return a - b;
    } else if (Array.isArray(a) && Array.isArray(b)) {
        if (a.length !== b.length) {
            return a.length - b.length;
        } else {
            for (let i = 0; i < a.length; i++) {
                const comparison = compare(a[i], b[i]);
                if (comparison !== 0) {
                    return comparison;
                }
            }
            return 0;
        }
    } else {
        throw new Error('Cannot compare different types');
    }
}

module.exports = compare;

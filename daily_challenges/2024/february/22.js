// Find the Town Judge

/**
 * @param {number} n
 * @param {number[][]} trust
 * @return {number}
 */
var findJudge = function(n, trust) {
       // check if the length is zero
       if (n === 1) {
        return 1;
    }
    // create a map to store the relations
    const relations = new Map();

    // set to store all the numbers 
    const everybody = new Set();

    // set to store the seen keys
    const seen = new Set();

    // itereate thtough all the relations
    for (const relation of trust) {
        const value = relation[1];
        const key = relation[0];
        // add them to everybody
        everybody.add(key);
        everybody.add(value);
        // add the key to seen
        seen.add(key);

        // now modify the existing relation of key
        if (relations.has(key)) {
            // it has been seen already, then
            const outgoing = relations.get(key);
            outgoing.add(value);
            // add it to the map again
            relations.set(key, outgoing);

        } else {
            // key has not been seen, create a new set
            const outgoing = new Set([value]);
            relations.set(key, outgoing);
        }

    } // end of for loop

    // now check if the seen and everybody differ by one
    if ((everybody.size - seen.size) === 1) {
        // there could be a judge
        // get the difference to see the possible judge
        const judge = difference(everybody, seen);
        if (judge === -1) {
            return judge;
        }
        // check if that value is present in every outgoing set of every key
            for (const [k, out] of relations.entries()) {
                if (!out.has(judge)) {
                    // it is not here
                    return -1;
                }
            } // iterated thorugh all the outgoing edges
            // he is the judge
            return judge;
        
    } else {
        // there is no possible judge
        return -1;
    }

};

function difference(everybody, seen) {
    
        // there is a 1 difference
        for (const v of everybody.values()) {
            // see if it was seen
            if (seen.has(v)) {
                // next one
                continue;
            }
            // we have a judge
            return v; 
        }
        // after iterating
        // we didnt find anyting
        return -1;
}

console.log(findJudge(3, [[1,3],[2,3]]));

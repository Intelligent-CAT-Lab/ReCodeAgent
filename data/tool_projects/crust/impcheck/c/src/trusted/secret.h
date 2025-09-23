
#pragma once

// This secret key is used to compute signatures for formulas, clauses, 
// and (un)satisfiability certificates. In principle, in order for the 
// trusted solving procedure to offer guarantees of soundness, the key 
// cannot be revealed to actors outside of the trusted modules. However, 
// if no malicious attacks are being considered, this is only relevant 
// in the sense that the process(es) interacting with trusted modules 
// should not have the particular key in memory (to rule out the 
// possibility of such an actor re-computing a signature). Under this
// pragmatic view, we just compile a constant key into the trusted
// modules and keep it at that. When considering malicious attacks,
// this data should instead be set dynamically for each solving attempt
// following some secure key exchange procedure.
extern const unsigned char SECRET_KEY[];

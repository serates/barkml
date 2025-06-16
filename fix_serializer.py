#!/usr/bin/env python3

import re

def fix_statement_file():
    with open('/Users/jmt/Projects/barkml/src/ser/statement.rs', 'r') as f:
        content = f.read()
    
    # Fix metadata cloning
    content = re.sub(r'self\.metadata\b', 'self.metadata.clone()', content)
    
    # Fix Statement::new_assign calls to handle Result
    content = re.sub(
        r'Statement::new_assign\(([^)]+)\)\s*\.map_err\([^)]+\)',
        r'Statement::new_assign(\1).map_err(|e| Error::Message { message: e.to_string() })',
        content
    )
    
    # Fix Statement::new_module calls to handle Result  
    content = re.sub(
        r'Statement::new_module\(([^)]+)\)\s*\.map_err\([^)]+\)',
        r'Statement::new_module(\1).map_err(|e| Error::Message { message: e.to_string() })',
        content
    )
    
    with open('/Users/jmt/Projects/barkml/src/ser/statement.rs', 'w') as f:
        f.write(content)

fix_statement_file()
print("Fixed statement serializer issues")

//
//  main.c
//  abc060_b
//
//  Created by Ding Chao on 4/29/17.
//  Copyright © 2017 Ding Chao. All rights reserved.
//

#include <stdio.h>

int main(int argc, const char * argv[]) {
    // insert code here...
    int A, B, C;
    scanf("%d%d%d", &A, &B, &C);
    int result = 0;
    for (int i = 0; i < B; i++) {
        int product = A * i; 
        int m = product % B;
        if (C == m) {
            result = 1;
            break;
        }
    }
    
    if (result) 
        printf("YES\n");
    else 
        printf("NO\n");
    return 0;
}
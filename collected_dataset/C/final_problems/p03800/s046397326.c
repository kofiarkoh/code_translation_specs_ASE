//
//  main.c
//  Main
//
//  Created by xiangwei wang on 2017/02/20.
//  Copyright © 2017年 xiangwei wang. All rights reserved.
//

#include <stdio.h>

#define MAX 100000
#define SHEEP 1
#define WOLF 2

int check(long n, long ans[MAX], const char s[MAX]) {
    for(long i = 1; i < n; i++) {
        if(ans[i] == SHEEP) {
            if(*(s + i) == 'o') {
                if(ans[(i + 1)%n] != 0 && ans[(i + 1)%n] != ans[i - 1]) {
                    return 1;
                }
                ans[i + 1] = ans[i - 1];
            } else {
                if(ans[(i + 1)%n] != 0 && ans[(i + 1)%n] != (ans[i - 1] == SHEEP ? WOLF : SHEEP)) {
                    return 1;
                }
                ans[i + 1] = ans[i - 1] == SHEEP ? WOLF : SHEEP;
            }
        } else {
            if(*(s + i) == 'o') {
                if(ans[(i + 1)%n] != 0 && ans[(i + 1)%n] != (ans[i - 1] == SHEEP ? WOLF : SHEEP)) {
                    return 1;
                }
                ans[i + 1] = ans[i - 1] == SHEEP ? WOLF : SHEEP;
            } else {
                if(ans[(i + 1)%n] != 0 && ans[(i + 1)%n] != ans[i - 1]) {
                    return 1;
                }
                ans[i + 1] = ans[i - 1];
            }
        }
    }
    
    return 0;
}
int main(int argc, const char * argv[]) {
    long n = 0;
    char s[MAX];
    long ans[MAX + 1];
    scanf("%ld", &n);
    scanf("%s", s);
    
    for(long i = 0; i < n; i++) {
        ans[i] = 0;
    }
    
    ans[0] = SHEEP;

    if(*(s) == 'o') {
        ans[1] = SHEEP;
        ans[n-1] = SHEEP;
    } else {
        ans[1] = WOLF;
        ans[n-1] = WOLF;
    }
    
    int result = check(n, ans, s);
    
    if(result == 1) {
        ans[0] = WOLF;
        if(*(s) == 'o') {
            ans[1] = WOLF;
            ans[n-1] = SHEEP;
            
            result = check(n, ans, s);
            if(result == 1) {
                ans[1] = SHEEP;
                ans[n-1] = WOLF;
                
                result = check(n, ans, s);
            }
            
        } else {
            ans[1] = WOLF;
            ans[n-1] = WOLF;
            result = check(n, ans, s);
            if(result == 1) {
                ans[1] = SHEEP;
                ans[n-1] = SHEEP;
                
                result = check(n, ans, s);
            }
            
        }
    }
    
    
    if(result == 1) {
        printf("-1");
    } else {
        for(int i = 0; i < n; i++) {
            printf("%s", ans[i] == 1 ? "S" : "W");
        }
    }
    printf("\n");
    return 0;
}

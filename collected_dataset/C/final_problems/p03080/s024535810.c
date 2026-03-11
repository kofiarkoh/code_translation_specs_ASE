#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>

int main(){
    int n,r_count=0,b_count=0;
    scanf("%d",&n);
    char s[n];
    scanf("%s",s);
    for (int i=0; i<n; i++) {
        if (s[i]=='R') {
            r_count++;
        }else{
            b_count++;
        }
    }
    if (r_count>b_count) {
        printf("Yes\n");
    }else{
        printf("No\n");
    }
    return 0;
}

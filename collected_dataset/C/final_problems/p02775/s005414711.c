#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdint.h>
#include<stdbool.h>
#include<tgmath.h>
#include<time.h>
#define ll long long
#define min(a,b) ((a)>(b)?(b):(a))
#define max(a,b) ((a)>(b)?(a):(b))
#define miin(a,b,c) min((a), min((b), (c)))
#define maax(a,b,c) max((a), max((b), (c)))

char s[1111111] = {};

int main(){
    scanf("%s", s);
    int l = strlen(s), sum = 0;
    for(int i=l-1;i;i--){
        if(s[i]<'5') sum += s[i] - '0';
        else if(s[i]>'5'){
            sum += '0' + 10 - s[i];
            s[i-1]++;
        }else {
            if(s[i-1]<'5'){
                sum += 5;
            }else {
                sum += 5;
                s[i-1]++;
            }
        }
    }
    sum += min(s[0] - '0', '0' + 11 - s[0]);
    printf("%d\n", sum);
    return 0;
}
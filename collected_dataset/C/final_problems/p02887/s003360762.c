#include<stdio.h>

int main(){
    int n;
    scanf("%d", &n);
    char s[n+1];
    scanf("%s", s);
    char pc = s[0];
    int ans = 1;
    int x = 1;
    while(s[x] != '\0'){
        if(s[x] != pc){
            ans++;
            pc = s[x];
        }
        x++;
    }
    printf("%d", ans);

    return 0;
}
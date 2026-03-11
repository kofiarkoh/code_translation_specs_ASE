#include<stdio.h>

int main()
{
    int n, i;
    char s[10001];
    
    scanf("%d", &n);
    getchar();
    scanf("%s", s);
    
    for(i=0; s[i]; ++i){
        s[i]-= 'A';
        s[i]+=n;
        s[i] %= 26;
        s[i]+='A';
    }
    printf(s);
    
    
    return 0;
}
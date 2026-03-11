#include<stdio.h>
int main(void){
    int n;
    char s[100];
    char t[100];
    scanf("%d",&n);
    scanf("%s%s",s,t);
    char a[100];
    for (int i = 0; i < n; i++)
    {
        a[2 * i] = s[i];
    }
    for (int i = 0; i < n; i++)
    {
        a[2 * i + 1] = t[i];
    }
    printf("%s\n",a);  
    return 0;
}
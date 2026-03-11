#include<stdio.h>
int main(void)
{
    char s[101],t[101],u[201];
    int i=0,j=0;
    scanf("%s%s",s,t);
    while(t[i]!='\0'){
        u[i]=t[i];
        i++;
    }
    while(s[j]!='\0'){
        u[i]=s[j];
        i++;
        j++;
    }
    u[i]='\0';
    printf("%s\n",u);
    return 0;
}
#include<stdio.h>
#include<string.h>
char s[11];
int main()
{
    gets(s);
    if(s[0]=='Y'&&s[1]=='A'&&s[2]=='K'&&s[3]=='I') 
    printf("Yes\n");
    else 
    printf("No\n");
    return 0;
}
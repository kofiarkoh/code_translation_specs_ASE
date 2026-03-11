#include<stdio.h>
#include<string.h>
int main()
{
    char s[1000001];
    scanf("%s",s);
    int len,flag=0;
    len=strlen(s);
//printf("%d\n",len);
    if(s[len-1]=='m' && s[len-2]=='a' && s[len-3]=='e' && s[len-4]=='r' && s[len-5]=='d')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='m' && s[len-4]=='a' && s[len-5]=='e' && s[len-6]=='r' && s[len-7]=='d')
    {
        flag=1;
    }
    else if(s[len-1]=='e' && s[len-2]=='s' && s[len-3]=='a' && s[len-4]=='r' && s[len-5]=='e')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='s' && s[len-4]=='a' && s[len-5]=='r' && s[len-6]=='e')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='s' && s[len-4]=='a' && s[len-5]=='r' && s[len-6]=='e' && s[len-7]=='m' && s[len-8]=='a' && s[len-9]=='e' && s[len-10]=='r' && s[len-11]=='d')
    {
        flag=1;
    }
    else if (s[len-1]=='e' && s[len-2]=='s' && s[len-3]=='a' && s[len-4]=='r' && s[len-5]=='e' && s[len-6]=='m' && s[len-7]=='a' && s[len-8]=='e' && s[len-9]=='r' && s[len-10]=='d')
    {
        flag=1;
    }
    else if(s[len-1]=='m' && s[len-2]=='a' && s[len-3]=='e' && s[len-4]=='r' && s[len-5]=='d' && s[len-6]=='r' && s[len-7]=='e' && s[len-8]=='m' && s[len-9]=='a' && s[len-10]=='e' && s[len-11]=='r' && s[len-12]=='d')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='m' && s[len-4]=='a' && s[len-5]=='e' && s[len-6]=='r' && s[len-7]=='d' && s[len-8]=='m' && s[len-9]=='a' && s[len-10]=='e' && s[len-11]=='r' && s[len-12]=='d')
    {
        flag=1;
    }
    else if(s[len-1]=='m' && s[len-2]=='a' && s[len-3]=='e' && s[len-4]=='r' && s[len-5]=='d' && s[len-6]=='e' && s[len-7]=='s' && s[len-8]=='a' && s[len-9]=='r' && s[len-10]=='e')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='m' && s[len-4]=='a' && s[len-5]=='e' && s[len-6]=='r' && s[len-7]=='d' && s[len-8]=='e' && s[len-9]=='s' && s[len-10]=='a' && s[len-11]=='r' && s[len-12]=='e')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='s' && s[len-4]=='a' && s[len-5]=='r' && s[len-6]=='e' && s[len-7]=='e' && s[len-8]=='s' && s[len-9]=='a' && s[len-10]=='r' && s[len-11]=='e')
    {
        flag=1;
    }
    else if(s[len-1]=='e' && s[len-2]=='s' && s[len-3]=='a' && s[len-4]=='r' && s[len-5]=='e' && s[len-6]=='r' && s[len-7]=='e' && s[len-8]=='m' && s[len-9]=='a' && s[len-10]=='e' && s[len-11]=='r' && s[len-12]=='d')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='s' && s[len-4]=='a' && s[len-5]=='r' && s[len-6]=='e' && s[len-7]=='r' && s[len-8]=='e' && s[len-9]=='m' && s[len-10]=='a' && s[len-11]=='e' && s[len-12]=='r' && s[len-13]=='d')
    {
        flag=1;
    }
    else if(s[len-1]=='m' && s[len-2]=='a' && s[len-3]=='e' && s[len-4]=='r' && s[len-5]=='d' && s[len-6]=='r' && s[len-7]=='e' && s[len-8]=='s' && s[len-9]=='a' && s[len-10]=='r' && s[len-11]=='e')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='s' && s[len-4]=='a' && s[len-5]=='r' && s[len-6]=='e' && s[len-7]=='e' && s[len-8]=='s' && s[len-9]=='a' && s[len-10]=='r' && s[len-11]=='e')
    {
        flag=1;
    }
    else if(s[len-1]=='r' && s[len-2]=='e' && s[len-3]=='m' && s[len-4]=='a' && s[len-5]=='e' && s[len-6]=='r' && s[len-7]=='d' &&s[len-8]=='r' && s[len-9]=='e' && s[len-10]=='s' && s[len-11]=='a' && s[len-12]=='r' && s[len-13]=='e' )
    {
        flag=1;
    }
    if(flag==0)
    {
        printf("NO\n");
    }
    else
    {
        printf("YES\n");
    }
    return 0;
}

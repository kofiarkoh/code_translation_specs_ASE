#include<stdio.h>
#include<string.h>
int main()
{
    char str[300];
    int l,i,j,a=0,count=0;
    scanf("%s",&str);
    l=strlen(str);
    for(i=0;i<l;i++)
    {
        a=0;
        for(j=0;j<l;j++)
        {
            if(str[i]==str[j]);
            {
               a++;
            }
        }
        if(a%2!=0&&a!=0||a==1&&a!=0)
            count++;
        if(count>0) {
                printf("No\n");
                return 0;
        }

    }
    printf("Yes\n");
    return 0;
}

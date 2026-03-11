#include <stdio.h>


int main(void)
{
    int k=0;
    char c,pre='0';
    while(1)
    {
        c=getchar();
        if(c=='\n'||c==EOF||c=='\0') break;
        if(c!=pre)
        {
            k++;
            pre=c;
        }
        else
        {
            pre='0';
            c=getchar();
            if(c=='\n'||c==EOF||c=='\0') break;
            k++;
        }
    }
    printf("%d",k);
    return 0;
}



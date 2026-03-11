#include <stdio.h>
#include <string.h>
#include <math.h>
int main()
{
    char s[4],l[4];

    while (~scanf ("%s%s",&s,&l))
    {
        int t=0;
        for (int i=2; i>=0; i--)
        {
            for(int j=0; j<3; j++)
                if(s[i]==l[j])
                    t++;
        }
        if(t==3)
        {
            printf("YES\n");
        }
        else
        {
            printf ("NO\n");
        }
    }
}

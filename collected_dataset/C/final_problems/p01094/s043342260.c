#include <stdio.h>

int main(void)
{
    int c[102];
    int n;
    char ch2[10];

    int max;
    int len, len2;

    

    

    int max2;

    int i, j, x;

    while (scanf("%d", &n) && n)
    {
            int v[30]={0};

            len=len2=-1;
            max=max2=0;
        
            for (int z=0; z<n; z++)
            {
                
                scanf("%s", ch2);

                c[z] = *ch2-'A';
            }

            // judge
            

            for (i=0, j=n; i<n; )
            {
                x = c[i];
                i++,j--;
                v[x]++;

                if (v[x] > max) 
                {
                    if (len < 0 || x == len)
                        max=v[x], len=x;
                    else max2 = max, len2 = len, max = v[x], len = x;
                
                }
                else if (v[x] > max2) max2= v[x], len2=x;
                //printf("%d + %d < %d\n", max2, j, max);
                if (max2 + j< max) break;
                
            }

            if (max==max2) puts("TIE");
            else printf("%c %d\n", len+'A', i);
        
    }
    return 0;
}

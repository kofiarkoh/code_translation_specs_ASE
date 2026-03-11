#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
int max(const void *a, const void *b){return *(int*)b - *(int*)a;}

int kmp[500001]={0};
void getKMP(char* str, int len)
{
    for(int i=1;i<len;i++)
    {
        int j;
        for(j=0;i+j<len;j++)
        {
            if(str[i+j] == str[j]) 
            {
                kmp[i+j] = (j+1) > kmp[i+j] ? (j+1) : kmp[i+j];
            }else break;
        }
        if(i+j==len) break; /* 末尾まで一致 */
    }
}

char s[500001];
char t[500001];
bool match[500001];
size_t sl,tl;

int main(void)
{
    
    scanf("%s",s);
    scanf("%s",t);
    sl = strlen(s);
    tl = strlen(t);
    getKMP(t,tl);

    {
        int i = 0;
        int j = 0;
        while(i<sl+tl)
        {
            if(s[i%sl]==t[j])
            {
                i++;
                j++;
                if(j==tl)
                {
                    match[i-tl] = true;
                    j = kmp[tl-1];
                }
            }else
            {
                if(j==0) i++;
                j = kmp[j];
            }
 //           printf("%d %d\n",i,j);
        }
    }

//    for(int i=0;i<sl;i++) printf("%d\n",match[i]);

    {
        int max;
        max = 0;
        for(int i=0;i<sl;i++)
        {
            int c,j;
            c = 0;
            j = i;
            while(match[j])
            {
                c++;
                if(c*tl >= 2*sl)
                {
                    printf("-1\n");
                    return 0;
                }
                j = (j + tl) % sl;
            }
            max = c > max ? c : max;
        }
        printf("%d\n",max);
    }
    return 0;
}

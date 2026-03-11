#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main()
{
 char s[500005];
 char temps[500005];
 unsigned long long Q;
 unsigned long long l[100001];
 unsigned long long r[100001];
 unsigned long long i;
 unsigned long long j;
 unsigned long long k;
 unsigned long long h;
 unsigned long long strlength;
 int doubleletter;
 unsigned long long maxstrlength;
 unsigned long long doubles[500000];

 scanf("%s",&s);
 scanf("%llu",&Q); 
 maxstrlength=0;
 for(i=0; i<Q; i++)
 {
         scanf("%llu %llu", &l[i], &r[i]);
         if(r[i]>maxstrlength) maxstrlength=r[i];
         l[i]=l[i]-1;
         r[i]=r[i]-1;
 }

 for(i=0; i<Q; i++)
 {
         strlength = r[i] - l[i] + 1;
         doubleletter=0;
         for(j=l[i]; j<r[i]+1; j++)
         {
           temps[j-l[i]] = s[j];
           if((j>l[i])&(doubleletter==0))
           {
              if(temps[j-l[i]]==temps[j-1-l[i]]) doubleletter=1;
           }
         }

         if(doubleletter==0)
         {
                 printf("No\n");
         }else{
                j=0;
                while((strlength>0)&(j<strlength))
                {
                        j++;
                        if(temps[j]==temps[j-1])
                        {
                                if(temps[j]=='z')
                                {
                                        for(k=j-1;k<strlength-2;k++)
                                        {
                                                temps[k]=temps[k+2];
                                        }
                                        j=0;
                                        strlength=strlength-2;
                                }else{
                                        temps[j-1]=temps[j-1]+1;
                                        for(k=j;k<strlength-1;k++)
                                        {
                                                temps[k]=temps[k+1];
                                        }
                                        j=0;
                                        strlength--;
                                }
                        }
                }
                if(strlength==0)
                {
                        printf("Yes\n");
                }else{
                        printf("No\n");
                }
        }
 }
 return 0;
}

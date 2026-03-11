#include<stdio.h>

int main(void)
{
   int n, l, i, f, j, g;
   char s[101][100], c;

   scanf("%d%d",&n,&l);
   for(i=0;i<n;i++)
      scanf("%s",&s[i]);

   for(i=0;i<n-1;i++)
      for(j=i+1;j<n;j++){
         f=0;
         while(s[i][f]==s[j][f])
            f++;
         if(s[i][f]>s[j][f])
            for(g=0;g<l;g++){
               c=s[i][g];
               s[i][g]=s[j][g];
               s[j][g]=c;
            }
      }

   for(i=0;i<n;i++)
      printf("%s",s[i]);
   printf("\n");
   return 0;
}
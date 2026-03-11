#include<stdio.h>
#include<string.h>
#include<stdlib.h>

int main()
{
        char a[10000],b[10000];
        int len1,len2,d,e,k=0,i,s[1000];
        gets(a);
        gets(b);
        len1=strlen(a);
        len2=strlen(b);
        if(len1>len2)
                printf("GREATER\n");
        else if(len1<len2)
                printf("LESS\n");
        else if(len1==len2)
        {
            for(i=0;i<len1;i++)
            {
                  d=a[i]-'0';
                  e=b[i]-'0';
                  if(d>e)
                  {
                      printf("GREATER\n");
                      break;
                  }
                  else if(d<e)
                  {
                          printf("LESS\n");
                           break;
                  }
                  else if(d==e)
                  {
                          s[k]=i;
                          k++;
                  }
            }
            //printf("%d\n",k);
            if(k==len1)
                printf("EQUAL\n");
        }

        return 0;

}

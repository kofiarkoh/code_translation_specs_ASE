#include<stdio.h>

int main(void)
{
        char S[11];
        int n, m, best, i;

        scanf("%s",S);
        best=999;
        for(i=0; i<11; i++){
                n=(S[i]-'0')*100+(S[i+1]-'0')*10+(S[i+2]-'0');
                m=n-753;
                if(m<0)m=m*(-1);
                if(best>m)
                        best=m;
        }
        printf("%d\n",best);

        return 0;
}

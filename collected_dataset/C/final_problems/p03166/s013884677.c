#include<stdio.h>
int main()
{
    int n,m;
    scanf("%d %d",&n,&m);
    int lpe[n+1]; //lp[i]はiで終わるlongest pathの長さ
    int snlpe[n+1]; //snlpe[i]はlpe[i]のstart node
    int lps[n+1]; //lp[i]はiで始まるlongest pathの長さ
    int enlps[n+1]; //snlpe[i]はlpe[i]のend node
    //initialize
    for (int i = 0; i < n+1; i++)
    {
        lpe[i]=0;
        snlpe[i]=i;
        lps[i]=0;
        enlps[i]=i;
    }
    for (int count = 0; count < m; count++)
    {
        int x,y;
        scanf("%d %d",&x,&y);
        if(lpe[enlps[y]]<lpe[x]+1+lps[y])
        {
            lpe[enlps[y]]=lpe[x]+1+lps[y];
            snlpe[enlps[y]]=snlpe[x];
        }
        if(lps[snlpe[x]]<lpe[x]+1+lps[y])
        {
            lps[snlpe[x]]=lpe[x]+1+lps[y];
            enlps[snlpe[x]]=enlps[y];
        }
    }
    int max=0;
    for (int i = 1; i < n+1; i++)
    {
        if(lps[i]>max)max=lps[i];
    }
    printf("%d\n",max);
    return 0;
}

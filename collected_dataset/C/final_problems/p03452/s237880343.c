#include <stdio.h>
int main(void)
{
    long int n,m;
    scanf("%ld %ld",&n,&m);
    long int l[200005]={};
    long int r[200005]={};
    int d[200005]={};
    for(int p=0;p<m;p++)scanf("%ld %ld %d",&l[p],&r[p],&d[p]);
    int gr[100005]={0};
    long long int pl[100005]={};
    long int rer[100005]={114514};
    long int rel[100005]={114514};
    long int c=0;
    long long int x=0;
    for(int q=0;q<m;q++)
    {
        rer[l[q]]=r[q];
        rel[r[q]]=l[q];
        if(gr[l[q]]==gr[r[q]] && gr[r[q]]!=0)
        {
            if(pl[r[q]]-pl[l[q]]!=d[q])
            {
                printf("No");
                return 0;
            }
        }
        else if(gr[l[q]]!=0 && gr[r[q]]!=0)
        {
            gr[r[q]]=gr[l[q]];
            long int len=pl[l[q]]+d[q]-pl[r[q]];
            x=r[q];
            while(x!=114514)
            {
                gr[x]=gr[l[q]];
                pl[x]=pl[x]+len;
                x=rer[x];
            }
            x=r[q];
            while(x!=114514)
            {
                gr[x]=gr[l[q]];
                pl[x]=pl[x]+len;
                x=rel[x];
            }
        }
        else if(gr[l[q]]!=0)
        {
            gr[r[q]]=gr[l[q]];
            pl[r[q]]=pl[l[q]]+d[q];
        }
        else if(gr[r[q]]!=0)
        {
            gr[l[q]]=gr[r[q]];
            pl[l[q]]=pl[r[q]]-d[q];
        }
        else
        {
            pl[l[q]]=0;
            pl[r[q]]=d[q];
            c++;
            gr[l[q]]=c;
            gr[r[q]]=c;
        }
    }
    long long int max=pl[0];
    long long int min=pl[0];
    for(int ppp=0;ppp<n;ppp++)
    {
        if(max<pl[ppp])max=pl[ppp];
        if(min>pl[ppp])min=pl[ppp];
    }
    if(max-min>1000000000)
    {
        printf("No");
        return 0;
    }
    printf("Yes");
}

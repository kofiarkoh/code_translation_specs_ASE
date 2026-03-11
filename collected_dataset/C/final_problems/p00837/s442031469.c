#include<stdio.h>
#define MAXN 1001

int book[101];
struct desk
{
    int num;
    int b[31];
}d[11];
struct student
{
    int cur;
    int num;
    int b[51];
}s[101];
int q[1001];
int m,c,n,elapse;

int main(void)
{
    int i, j, k, t, maxbook, maxtime, cur_book, take_book;
    while(scanf("%d %d %d",&m,&c,&n), m||c||n)
    {
        k=0;elapse=0;
        for(i=1;i<=m;i++)
            d[i].num=0;
        for(i=0;i<=100;i++)
            book[i]=m+1;
        for(i=0;i<n;i++)
        {
            scanf("%d",&s[i].num);
            for(j=0;j<s[i].num;j++)
                scanf("%d",&s[i].b[j]);
            s[i].cur=0;
            if(s[i].num)
                q[k++]=i;
        }
        j=0;
        while(j!=k)
        {
            cur_book = s[q[j]].b[s[q[j]].cur++];
            elapse += book[cur_book];
            if(book[cur_book]!=m+1)
            {
                t=book[cur_book];
                for(i=0; i<d[t].num; i++)
                    if(d[t].b[i]==cur_book)
                        break;
                for( ; i<d[t].num; i++)
                    d[t].b[i]=d[t].b[i+1];
                d[t].num--;
            }
            if(d[1].num==c)
            {
                i=2;
                while(i<=m && d[i].num==c)
                    i++;
                if(i>m)
                    book[cur_book]=m+1,elapse=elapse+m+1;
                else
                    d[i].b[d[i].num++]=cur_book, book[cur_book]=i, elapse+=i;
                take_book=d[1].b[0];
                for(i=0; i<d[1].num; i++)
                    d[1].b[i]=d[1].b[i+1];
                elapse++;
                d[1].num--;
                i=2;
                while(i<=m && d[i].num==c)
                    i++;
                if(i>m)
                    book[take_book]=m+1, elapse=elapse+m+1;
                else
                    d[i].b[d[i].num++]=take_book, book[take_book]=i, elapse+=i;
                elapse += book[cur_book];
                if(book[cur_book]!=m+1)
                    d[book[cur_book]].num--;
            }
            d[1].b[d[1].num++]=cur_book;
            book[cur_book]=1;
            elapse++;
            if(s[q[j]].cur<s[q[j]].num)
                q[k]=q[j], k=(k+1)%MAXN;
            j=(j+1)%MAXN;
        }
        printf("%d\n",elapse);
    }
    return 0;
}
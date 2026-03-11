#include<stdio.h>

#define MAX 100
#define N 100
#define WHITE 0
#define GRAY  1
#define BLACK 2

int isEmpty();
int isFull();
void enqueue(int);
int dequeue();
void bfs(int,int);

int head = 0, tail = 0;
int que[MAX];
int G[N][N];
char color[N];
int d[N];
int f[N];
int main()
{
    int n,i,k,u,v,j;
    
    scanf("%d", &n);
    
    for(i=0;i<n;i++)
    {
        scanf("%d%d", &u, &k);
        for(j=0;j<k;j++)
        {
            scanf("%d", &v);
            G[u-1][v-1]=1;
        }
    }
    
    bfs(0,n);
    
    for(i=0;i<n;i++)
    {
        printf("%d %d\n", i+1, d[i]);
    }
    return 0;
}

int isEmpty()
{
    return head == tail;
}
int isFull()
{
    return head == (tail+1)%MAX;
}
void enqueue(int x)
{
    if(isFull())
    {
        printf("OVF\n");
    }
    que[tail] = x;
    if(tail+1 == MAX)
    {
        tail = 0;
    }
    else
    {
        tail++;
    }
}
int dequeue()
{
    if(isEmpty())
    {
        printf("UNF\n");
    }
    int x = que[head];
    if(head+1 == MAX)
    {
        head = 0;
    }
    else
    {
        head++;
    }
    return x;
}
void bfs(int s, int n)
{
    int i,u;
    for(i=0;i<n;i++)
    {
        color[i]=WHITE;
        d[i]=-1;
    }
    color[s]=GRAY;
    d[s]=0;
    enqueue(s);
    while(!isEmpty())
    {
        u=dequeue();
        for(i=0;i<n;i++)
        {
            if(G[u][i])
            {
                if(color[i]==WHITE)
                {
                    color[i]=GRAY;
                    d[i]=d[u]+1;
                    enqueue(i);
                }
            }
        }
        color[u]=BLACK;
    }
}
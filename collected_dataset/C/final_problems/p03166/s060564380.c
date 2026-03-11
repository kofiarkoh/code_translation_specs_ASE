#include<stdio.h>
#include<stdlib.h>

struct node {
	int val;
	struct node* next;
};
struct node* a[100005];
void insert (int u,int v) {
        struct node* q;
        q=(struct node*)malloc(1*sizeof(struct node));
        q->val=v;
        q->next=NULL;
        if(a[u]==NULL)
                a[u]=q;
        else {
                struct node* p;
                p=a[u];
                q->next=p;
                a[u]=q;
        }
}

int max(int a, int b)
{
    if(a<b)
        return b;
    else
        return a;
}

int main()
{
    int m,n,u,v,visited[100005],arr[1000005],count[100005],count2[100005],j,maxi,i,l,r;
    struct node* p;
    scanf("%d %d",&n,&m);
    for(i=1;i<=n;i++)
    {
        visited[i]=0;
        count[i]=0;
        count2[i]=0;
    }
    for(i=0;i<m;i++)
    {
        scanf("%d %d",&u,&v);
        count2[v]++;
        insert(u,v);
    }
    for(i=1;i<=n;i++)
    {
        l=0,r=-1;
        if(count2[i]==0 && visited[i]==0)
        {
            visited[i]=1;
            p=a[i];
            while(p!=NULL)
            {
                count2[p->val]--;
                if(count2[p->val]==0)
                {
                    arr[++r]=p->val;
                }
                count[p->val]=max(count[p->val],count[i]+1);
                p=p->next;
            }
            while(l<=r)
            {
                j=arr[l];
                visited[j]=1;
                p=a[j];
                while(p!=NULL)
                {
                    count2[p->val]--;
                    if(count2[p->val]==0)
                    {  
                        arr[++r]=p->val;
                    }
                    count[p->val]=max(count[p->val],count[j]+1);
                    p=p->next;
                }
                l++;
            }
        }
    }
    maxi=-1;
    for(i=1;i<=n;i++)
    {
        if(count[i]>maxi)
        {
            maxi=count[i];
        }
    }
    printf("%d\n",maxi);
}
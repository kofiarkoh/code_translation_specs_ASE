#include<stdio.h>
double min,max,t,P,ans;
int k;
void find(double l,double r,double p,int cur)
{
	double mid;
    if(r<=min||l>=max) 
		return;   
    if(l>=min&&r<=max) 
	{ ans+=p; return; }
    mid=(l+r)/2;
    if(cur==k)
	{
        if(mid<=max&&mid>=min) 
			ans+=p;
        return ;
    }
    if(mid>=t)
	{
        find(l,mid,p*(1-P),cur+1);
        find(mid,r,p*P,cur+1);
    }
    else
	{        
        find(l,mid,p*P,cur+1);
        find(mid,r,p*(1-P),cur+1);
    }
	return ;
}

int main()
{
    double l,r,p,e;
    scanf("%d%lf%lf",&k,&l,&r);
        
        scanf("%lf %lf %lf",&P,&e,&t);
		ans=0;
        min=t-e;
		max=t+e;
        find(l,r,1,0);
        printf("%.6lf\n",ans);
    
    return 0;
}
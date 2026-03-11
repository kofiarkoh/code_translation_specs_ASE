#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>
#define max(x,y)(x>y?x:y)
#define min(x,y)(x>y?y:x)
typedef long long int ll;
typedef unsigned long long int ull;
int asc_cmp(const void* X,const void* Y){int x=*(int*)X,y=*(int*)Y;if(x<y)return -1;else if(x==y)return 0;else return 1;}
int desc_cmp(const void* X,const void* Y){int x=*(int*)X,y=*(int*)Y;if(x<y)return 1;else if(x==y)return 0;else return -1;}
int main(){
    ull n,m;scanf("%llu %llu",&n,&m);
    if(n==1&&m==1)printf("0\n");
    else if(n==1||m==1)printf("%llu\n",n*m-2);
    else printf("%llu\n",n*m-((n-2)*2+m*2));
    return 0;

}
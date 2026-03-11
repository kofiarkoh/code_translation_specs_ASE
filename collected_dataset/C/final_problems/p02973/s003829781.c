/*よくばりセット2019/07/13 */
#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#define MOD 1000000007
typedef long long int ll;
void llswap(ll *x,ll *y){ll temp;temp=*x;*x=*y;*y=temp;}
void swap(int *x,int *y){int tmp;tmp=*x;*x=*y;*y=tmp;}
int max(int x,int y){return x>y?x:y;}
int min(int x,int y){return x>y?y:x;}
ll llmax(ll x,ll y){return x>y?x:y;}
ll llmin(ll x,ll y){return x>y?y:x;}
int asc(const void *a,const void *b){return *(int*)a-*(int*)b;}
int desc(const void *a,const void *b){return *(int*)b-*(int*)a;}

int main(){
    static int n,a[400000],list[400000]={};
    scanf("%d",&n);
    for(int i=0;i<n;i++){
        scanf("%d",&a[i]);
    }

    int cnt=0;
    for(int i=0;i<n;i++){
        if(list[i]==-1)continue;
        list[i]=-1;
        for(int j=i+1;j<n;j++){
            if(list[j]==-1)continue;
            if(a[i]<a[j]){
                list[j]=-1;
            }
        }
        
        cnt++;
    }
    printf("%d\n",cnt);
    return 0;

}
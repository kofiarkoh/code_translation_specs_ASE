#include<stdio.h>
#include<math.h>
#include<limits.h>
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define LARGE 100001

int a[LARGE]={0};
int Add[LARGE]={0};
int Sum[LARGE]={0};
int b,n;

int main(){

	int q,i,j,com;
	int x,y,z;
	int sum=0;
	
	scanf("%d %d",&n,&q);
	
	b=sqrt(n);
	
	for(j=0;j<q;j++){
	
		scanf("%d",&com);
		
		if(com==1){/*sum*/
			sum=0;
			scanf("%d %d",&x,&y);
			if(y/b-x/b<2){
				for(i=x;i<=y;i++){
					sum+=a[i]+Add[i/b];
				}
			}
			else {
				for(i=x;i<=MIN(y,(x/b+1)*b-1);i++)sum+=a[i]+Add[i/b];
							
				for(i=x/b+1;i<y/b;i++)sum+=Sum[i];
			
				for(i=MAX(x,y/b*b);i<=y;i++)sum+=a[i]+Add[i/b];
				
			}
			
			printf("%d\n",sum);
		}
		
		else {/*add*/
			scanf("%d %d %d",&x,&y,&z);
			if(y/b-x/b<2){
				for(i=x;i<=y;i++){
					a[i]+=z;
					Sum[i/b]+=z;
				}
			}
			else {
				for(i=x;i<=MIN(y,(x/b+1)*b-1);i++){
					a[i]+=z;
					Sum[i/b]+=z;
				}
			
				for(i=x/b+1;i<y/b;i++){
					Add[i]+=z;
					Sum[i]+=b*z;
				}
			
				for(i=MAX(x,y/b*b);i<=y;i++){
					a[i]+=z;
					Sum[i/b]+=z;
				}
				
			}
		}
		
	}
	
	return 0;
	
}

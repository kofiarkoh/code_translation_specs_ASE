#include <stdio.h>

void search(int);

// 経路
int path[1001];
int check1=0, check2=0;
int a[1001][2], e, start, c=0;

int main(void)
{
	int i,j,v,s,t,next;
	
	scanf("%d %d", &v,&e);  	
		for(i=0; i<e;i++){
		scanf("%d %d", &s,&t);  
		a[i][0] = s;
		a[i][1] = t;
			}
			
			for(i=0; i<e;i++){
				start = a[i][0];
				next = a[i][1];
				path[0] = i;
				c=1;
				
				search(next);
			}
			
				if(check2==1) printf("1\n");
				else printf("0\n");
				
				return 0;
}
				
void search(int next){
 int i,j,check;
 check2=0;
 check=0;
 
 for(i=0;i<e;i++){
 	if(next==a[i][0]){
 		check=1;
 		check1=0;
 		for(j=0;j<c;j++){
 			if(i==path[j]) check1=1;
 		}	
 		if(check1!=1){
 		next=a[i][1];
 		path[c]=i;
 		c++;
 		}
 	}
 	if(next==start){
 			check2=1;
 			break;
 		}
 		else if(check==1)search(next);
 	}
  c--;			
}



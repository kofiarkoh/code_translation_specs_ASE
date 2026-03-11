#include <stdio.h>

int v[2002];
int a[1001][1001] ;
int k;
int check=0;
int v2;




void visit(int i)
{
int j;
v[i] = 1;
for (j=0 ; j<v2 ; j++){
if (a[i][j] == 1 && k==j){
	check = 1;
	break;
}
if (a[i][j] == 1 && v[j] == 0){
visit(j);
}
}
}

int main(){

 int e,s,t,i,j;
		scanf("%d %d", &v2,&e);  	
		for(i=0; i<v2;i++){
			for(j=0; j<v2;j++){
				a[i][j]=0;
			}
		}
	for(i=0; i<e;i++){
		scanf("%d %d", &s,&t);  
		a[s][t] = 1;
			}
for (k = 0 ; k < v2 ; k++){
	check =0;
for(i=0 ; i<v2 ; i++)
v[i]=0;
visit(k);

 }
 
 if(check==1) printf("1\n");
				else printf("0\n");
				
return (0); 
}



#include <stdio.h>
#include <stdlib.h>
int main() {
	int a,b;
	scanf("%d %d",&a,&b);
	int vetor[2*a],vet[2*b];
	for(int x=0;x<2*a;++x)
	    scanf("%d",&vetor[x]);
	for(int z=0;z<2*b;++z)
	    scanf("%d",&vet[z]);
	
	for(int f=0;f<2*a;f+=2){
	    int val = 0,v=1000000000,pos=0;
	    for(int g=0;g<2*b;g+=2){
	        val=abs(vetor[f]-vet[g])+abs(vetor[f+1]-vet[g+1]);
	        if(val<v){
	            v= val;
	            pos=(g/2)+1;
	        }
	    }
	    printf("%d\n",pos);
	}    
	return 0;
}
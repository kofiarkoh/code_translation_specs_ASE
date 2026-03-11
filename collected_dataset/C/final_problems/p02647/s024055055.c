#include <stdio.h>
#include <stdlib.h>

int main(void){
    int n,k,a[300000];
    int light[300000];
    int i,j,p;
    
    scanf("%d",&n);
    scanf("%d",&k);
    for(i = 1; i<n+1; i++){
    	scanf("%d%",&a[i]);
    }
    
    
    //light update
    
    for(p = 0; p<k; p++){
       	for(i = 1; i<n+1; i++){
    		light[i] = 0;
    	}
    	
    	for(i = 1; i<n+1; i++){
    		//printf("n = %d\n",i);
	    	for(j=0; j<a[i]+1; j++){
	    		if((i-j)>0){
	    			light[i-j] =light[i-j]+1;
	    			//printf("a[%d]= %d\n",(i-j),light[i-j]);
	    		}
	    		if((i+j)<(n+1)){
	    			light[i+j] = light[i+j]+1;
	    			//printf("a[%d]= %d\n",(i+j),light[i+j]);
	    		}
	    	}
	    	light[i] = light[i]-1;  
    		//printf("finish a[%d]= %d\n",i,light[i]);
    	}
    	for(i = 1; i<n+1; i++){
    		a[i] = light[i];
    		//printf("update a=%d",a[i]);
    	}
    	
    }
    for(i = 1; i<n+1; i++){
    	printf("%d %",light[i]);
    }


    

    
    printf("\n");
    
}
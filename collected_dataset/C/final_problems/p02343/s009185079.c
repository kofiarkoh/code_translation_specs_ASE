#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10001

struct DSL1A{
	
	short *values;
	short len;
	
};
typedef struct DSL1A DSL1A;

DSL1A *array;
int n;

DSL1A newData(int num);
DSL1A cat(DSL1A a, DSL1A b);

int isFound(DSL1A data, int n);
int getIdx(int num);

void uniteData(int idx1, int idx2);

void display(DSL1A data);

int main(){
	
	int q;
	int i, k;
	int com, x, y;
	int output;
	int idx1, idx2;
	char temp[100];
	
	scanf("%d %d", &n, &q);
	
	array = (DSL1A*)malloc((sizeof(DSL1A)*n)+1);
	
	for(i=0; i<n; i++){
		
		array[i] = newData(i);
		
		//display(array[i]);
		
	}
	
	
	for(i=0; i<q; i++){
		
		scanf("%d %d %d", &com, &x, &y);
		
		//do unite
		if(com == 0){
			
			idx1 = getIdx(x);
			idx2 = getIdx(y);
			
			//printf("uniteData(%d, %d)\n", idx1, idx2);
			
			uniteData(idx1, idx2);
			
			//printf("united::");
			//display(array[idx1]);
			//puts("");
			
		}
		
		//do same
		else{
			
			output = 0;
			
			for(k=0; k<n; k++){
				
				if(isFound(array[k], x)){
					
					if(isFound(array[k], y)){
						
						output = 1;
						
					}
					
				}
				
			}
			
			printf("%d\n", output);
			
		}
		
	}
	
	
	for(i=0; i<n; i++){
		
		free(array[i].values);
		
	}
	
	free(array);
	
	return 0;
	
}



DSL1A newData(int num){
	
	DSL1A ret;
	
	ret.values = (short*)calloc(n+1, sizeof(short));
	ret.values[0] = num;
	
	ret.len = 1;
	
	//snprintf(ret.values, sizeof(ret.values), "%d", num);
	
	
	return ret;
	
}



int isFound(DSL1A data, int num){
	
	int i;
	int ret = 0;
	
	for(i=0; i<data.len; i++){
		
		if(data.values[i] == num){
			
			ret = 1;
			break;
			
		}
		
	}
	
	return ret;
	
}



int getIdx(int num){
	
	int i;
	
	for(i=0; i<n; i++){
		
		if(isFound(array[i], num)){
			
			return i;
			
		}
		
	}
	
	
}


void uniteData(int idx1, int idx2){
	
	int i;
	DSL1A temp;
	
	if(idx1 == idx2){
		
		return;
		
	}
	
	//printf("do cat\n");
	
	for(i=0; i<array[idx2].len; i++){
		
		array[idx1].values[array[idx1].len++] = array[idx2].values[i];
		
	}
	
	
	//array[idx1].len = strlen(array[idx1].values);
	
	free(array[idx2].values);
	array[idx2].values = NULL;
	
	//array[idx2].values[0] = '\0';
	
	array[idx2].len = 0;
	
}


DSL1A cat(DSL1A a, DSL1A b){
	
	int i;
	
	
	display(a);
	display(b);
	
	for(i=0; i<b.len; i++){
		
		a.values[a.len++] = b.values[i];
		
	}
	
	return a;
	
}



void display(DSL1A data){
	
	int i;
	
	for(i=0; i<data.len; i++){
		
		printf("%d", data.values[i]);
		
	}
	
	printf(", %d\n", data.len);
	
	
}
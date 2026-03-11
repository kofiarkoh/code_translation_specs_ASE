#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

//Global Variable
int n; // the size of target array
int q; // the number of querys
int* D;// the minimum value array.
int* lazy;
int x;
int i; // the variable for for loop. just lazy.
//End of Global Variable



// Function prototype Area
void __get_init_value(void);
int __min(int a, int b);
void initRMQ( int n_);
void update(int a, int b);
void __update(int a, int b, int k, int l, int r);
int findMin(int a,int b);
void __find(int k);
int query(int a , int b, int k, int l, int r);

void lazy_evaluate(int k);
// End of Funtion prototype Area


int main(int argc, char* argv[]){

	__get_init_value();
	//printf("init n = %d\n",n);
	initRMQ(n);
	//printf("after initRMQ n = %d\n",n);


	int com;
	int s;
	int t;
	//int x;
	int ans;
	while(q>0){
		scanf("%d",&com);
		if(com){
			scanf("%d %d",&s,&t);
			ans = findMin(s,t);
			printf("%d\n",ans);
		}else{
			scanf("%d %d %d",&s, &t, &x);
			update(s,t);
		}	
		q--;
	}

	return 0;
}

int __min(int a, int b){
	if(a > b){
	return b;
	}else{
	return a;
	}
}

void __get_init_value(void){

	scanf("%d %d", &n, &q);
	return;
}//end of get init value;

void initRMQ(int n_){
	// n_ is used to preserved original n;
	// after this function, n is not the total item number anymore.

	n = 1;
	while(n < n_){
		n *= 2;
	}
	
	D = malloc(sizeof(int)* (2*n-1));
	lazy = malloc(sizeof(int)* (2*n-1));
	if(D == NULL){
		printf("Init Array Failed, no memory or size is wrong\n");
		return;
	}

	for(i = 0; i< 2*n-1 ; i++){
		D[i] = INT_MAX;
		lazy[i] = INT_MAX;
	}
	return;
}// end of initRMQ;


void update(int a , int b){
	__update(a, b+1, 0, 0, n);
}// end of update;

void __update(int a, int b, int k , int l, int r){
	//printf("qurey a=%d b=%d k=%d l=%d r=%d\n",a,b,k,l,r);
	//
	if(k >= 2*n-1){return;}

	lazy_evaluate(k);

	if(r <= a || b <= l){
		return;
	}
	if(a <= l && r <= b){
		D[k] = x;
		if (l == r){
			return;
		}else{	
			if(k < n-1){
				lazy[k*2+1] = x;
				lazy[k*2+2] = x;
			}
			return;
		}
	}
	//printf("(L+R)/2 = %d\n",(l+r)/2);
	//printf("vl, l=%d\n",l);
	__update(a, b, k*2+1, l , (l+r)/2);
	//printf("vr, r=%d\n",r);
	__update(a, b, k*2+2, (l+r)/2, r);
	D[k] =  __min(D[k*2+1],D[k*2+2] );

}


int findMin(int a, int b){
	//printf("findMin n = %d\n",n);
	//a += n - 1;
	//__find(a);
	//return D[a];
	return query( a, b+1, 0, 0, n);
}// end of findMin;

void __find(int k){
	if(k > 0){
		__find((k-1)/2);
	}
	lazy_evaluate(k);

}

int query(int a, int b, int k , int l , int r){
	//printf("qurey a=%d b=%d k=%d l=%d r=%d\n",a,b,k,l,r);
	lazy_evaluate(k);
	if(r <= a || b <= l){
		return INT_MAX;
	}
	if(a <= l && r <= b){
		return D[k];
	}
	//printf("(L+R)/2 = %d\n",(l+r)/2);
	//printf("vl, l=%d\n",l);
	int vl = query(a, b, k*2+1, l , (l+r)/2);
	//printf("vr, r=%d\n",r);
	int vr = query(a, b, k*2+2, (l+r)/2, r);
	return __min(vl, vr);
}//end of query

void lazy_evaluate(int k){
	if( lazy[k] == INT_MAX){
		return;
	}
	D[k] = lazy[k];
	if(k < n-1){
		lazy[2*k +1] = lazy[k];
		lazy[2*k +2] = lazy[k];
	}
	lazy[k] = INT_MAX;
}


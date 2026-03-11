#include<stdio.h>
#include<stdlib.h>
static const int MAX_N = 1000;

struct Node{
	int data;
	struct Node *under;
};
struct Stack{
	struct Node *top;
};

struct Stack *CreateStack(){
	struct Stack *S = malloc(sizeof(struct Stack));
	S->top = NULL;
	return S;
}
void push(struct Stack *S, int x){
	if(S->top == NULL){
		S->top = malloc(sizeof(struct Node));
		S->top->data = x;
		S->top->under = NULL;
	}else{
		struct Node *bf = malloc(sizeof(struct Node));
		bf->under = S->top;
		bf->data = x;
		S->top = bf;
	}
}
void top(struct Stack *S){
	if(S->top == NULL) return;
	printf("%d\n", S->top->data);
}
void pop(struct Stack *S){
	if(S->top == NULL) return;
	if(S->top->under == NULL){
		free(S->top);
		S->top = NULL;
		return;
	}
	struct Node *bf = S->top->under;
	free(S->top);
	S->top = bf;
}
void deleteStack(struct Stack *S){
	if(S->top == NULL) return;
	struct Node *cur = S->top;
	for(;;){
		if(cur->under == NULL){
			free(cur);
			break;
		}
		struct Node *bf = cur->under;
		free(cur);
		cur = bf;
	}
	free(S);
}

int main(){
	int n, q;
	scanf("%d %d", &n, &q);
	struct Stack *S[MAX_N];
	for(int i = 0; i < n; i++) S[i] = CreateStack();
	for(int i = 0; i < q; i++){
		int op, t;
		scanf("%d %d", &op, &t);
		if(op == 0){
			int x;
			scanf("%d", &x);
			push(S[t], x);
		}else if(op == 1){
			top(S[t]);
		}else if(op == 2){
			pop(S[t]);
		}
	}
	for(int i = 0; i < n; i++) deleteStack(S[i]);
	return 0;
}


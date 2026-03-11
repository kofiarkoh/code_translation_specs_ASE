#include <stdio.h>
#include <string.h>

#if 0
#define DEBUG_MODE
#endif

#define TREE_MAX 40000

typedef struct {
	int key_r;
	int key_c;
	int data;
	int isred;
	int parent;
	int left;
	int right;
} rbt_node_t;

int rbt_node_used;
rbt_node_t rbt_node[TREE_MAX+1];

void rbt_init(void) {
	rbt_node_used=1;
	memset(rbt_node,0,sizeof(rbt_node));
	rbt_node[0].key_r=-0x7fffffff;
	rbt_node[0].key_c=-0x7fffffff;
}

int rbt_search(int key_r,int key_c) {
	int pos=rbt_node[0].right;
	while(pos>0) {
		if(rbt_node[pos].key_r==key_r && rbt_node[pos].key_c==key_c) {
			return rbt_node[pos].data;
		} else if(rbt_node[pos].key_r<key_r ||
				(rbt_node[pos].key_r==key_r && rbt_node[pos].key_c<key_c)) {
			pos=rbt_node[pos].right;
		} else {
			pos=rbt_node[pos].left;
		}
	}
	return -1;
}

int rbt_insert(int key_r,int key_c,int data) {
	int pos=0;
	int parent,p_parent,p_left,p_right;
	int pp_parent,pp_left,pp_right;
	while(1) {
		if(rbt_node[pos].key_r==key_r && rbt_node[pos].key_c==key_c) {
			rbt_node[pos].data=data;
			return 1;
		} else if(rbt_node[pos].key_r<key_r ||
				(rbt_node[pos].key_r==key_r && rbt_node[pos].key_c<key_c)) {
			if(rbt_node[pos].right==0)break;
			pos=rbt_node[pos].right;
		} else {
			if(rbt_node[pos].left==0)break;
			pos=rbt_node[pos].left;
		}
	}
	if(rbt_node_used>TREE_MAX)return 0;
	rbt_node[rbt_node_used].key_r=key_r;
	rbt_node[rbt_node_used].key_c=key_c;
	rbt_node[rbt_node_used].data=data;
	rbt_node[rbt_node_used].isred=1;
	rbt_node[rbt_node_used].parent=pos;
	rbt_node[rbt_node_used].left=0;
	rbt_node[rbt_node_used].right=0;
	if(rbt_node[pos].key_r<key_r ||
			(rbt_node[pos].key_r==key_r && rbt_node[pos].key_c<key_c)) {
		rbt_node[pos].right=rbt_node_used;
	} else {
		rbt_node[pos].left=rbt_node_used;
	}
	pos=rbt_node_used;
	rbt_node_used++;

	while(pos>0) {
		if(rbt_node[pos].parent==0) {
			rbt_node[pos].isred=0;
			break;
		}
		parent=rbt_node[pos].parent;
		p_parent=rbt_node[parent].parent;
		p_left=rbt_node[parent].left;
		p_right=rbt_node[parent].right;
		if(!rbt_node[parent].isred) {
			break;
		}
		pp_parent=rbt_node[p_parent].parent;
		pp_left=rbt_node[p_parent].left;
		pp_right=rbt_node[p_parent].right;
		if(rbt_node[pp_left].isred && rbt_node[pp_right].isred) {
			rbt_node[pp_left].isred=0;
			rbt_node[pp_right].isred=0;
			rbt_node[p_parent].isred=1;
			pos=p_parent;
		} else {
			if(pos==p_right && parent==pp_left) {
				if(rbt_node[pos].left>0) {
					rbt_node[rbt_node[pos].left].parent=parent;
				}
				rbt_node[parent].right=rbt_node[pos].left;
				rbt_node[parent].parent=pos;
				rbt_node[pos].left=parent;
				rbt_node[pos].parent=p_parent;
				rbt_node[p_parent].left=pos;
				pos=parent;
			} else if(pos==p_left && parent==pp_right) {
				if(rbt_node[pos].right>0) {
					rbt_node[rbt_node[pos].right].parent=parent;
				}
				rbt_node[parent].left=rbt_node[pos].right;
				rbt_node[parent].parent=pos;
				rbt_node[pos].right=parent;
				rbt_node[pos].parent=p_parent;
				rbt_node[p_parent].right=pos;
				pos=parent;
			} else if(pos==p_left && parent==pp_left) {
				if(rbt_node[pp_parent].left==p_parent) {
					rbt_node[pp_parent].left=parent;
				} else if(rbt_node[pp_parent].right==p_parent) {
					rbt_node[pp_parent].right=parent;
				} else {
					/* error!bug! */
					return 0;
				}
				if(rbt_node[parent].right>0) {
					rbt_node[rbt_node[parent].right].parent=p_parent;
				}
				rbt_node[p_parent].left=rbt_node[parent].right;
				rbt_node[p_parent].parent=parent;
				rbt_node[parent].right=p_parent;
				rbt_node[parent].parent=pp_parent;
				rbt_node[p_parent].isred=1;
				rbt_node[parent].isred=0;
				break;
			} else if(pos==p_right && parent==pp_right) {
				if(rbt_node[pp_parent].left==p_parent) {
					rbt_node[pp_parent].left=parent;
				} else if(rbt_node[pp_parent].right==p_parent) {
					rbt_node[pp_parent].right=parent;
				} else {
					/* error!bug! */
					return 0;
				}
				if(rbt_node[parent].left>0) {
					rbt_node[rbt_node[parent].left].parent=p_parent;
				}
				rbt_node[p_parent].right=rbt_node[parent].left;
				rbt_node[p_parent].parent=parent;
				rbt_node[parent].left=p_parent;
				rbt_node[parent].parent=pp_parent;
				rbt_node[p_parent].isred=1;
				rbt_node[parent].isred=0;
				break;
			} else {
				/* error!bug! */
				return 0;
			}
		}
	}
	rbt_node[0].parent=0;
	rbt_node[0].left=0;
	rbt_node[0].isred=0;
	return 1;
}

int N,Q,A,B,C,D,E,F,G;

int rc_flip,row_flip,col_flip;
int row_table[40001];
int col_table[40001];

int get_value(int r,int c) {
	int kari_value;
	int hr,hc,temp;
	hr=r;hc=c;
	if(rc_flip){temp=hr;hr=hc;hc=temp;}
	hr=row_flip?row_table[N-hr+1]:row_table[hr];
	hc=col_flip?col_table[N-hc+1]:col_table[hc];
#ifdef DEBUG_MODE
	printf("[%d,%d]",hr,hc);
#endif
	kari_value=rbt_search(hr,hc);
	if(kari_value<0)return (int)(((long long)hr*A+(long long)hc*B)%C);
	return kari_value;
}

void set_value(int r,int c,int v) {
	int hr,hc,temp;
	hr=r;hc=c;
	if(rc_flip){temp=hr;hr=hc;hc=temp;}
	hr=row_flip?row_table[N-hr+1]:row_table[hr];
	hc=col_flip?col_table[N-hc+1]:col_table[hc];
	rbt_insert(hr,hc,v);
}

int main(void) {
	int i;
	char order[4];
	int h1,h2,h3,h4;
	int temp;
	int r,c;
	long long hash;
	scanf("%d%d%d%d%d%d%d%d%d",&N,&Q,&A,&B,&C,&D,&E,&F,&G);
	rbt_init();
	rc_flip=row_flip=col_flip=0;
	for(i=1;i<=N;i++) {
		row_table[i]=col_table[i]=i;
	}
	for(i=0;i<Q;i++) {
#ifdef DEBUG_MODE
		for(r=D;r<=E;r++) {
			for(c=F;c<=G;c++) {
				printf("%3d",get_value(r,c));
			}
			puts("");
		}
#endif
		scanf("%s",order);
		if(strcmp(order,"WR")==0) {
			scanf("%d%d%d",&h1,&h2,&h3);
			set_value(h1,h2,h3);
		} else if(strcmp(order,"CP")==0) {
			scanf("%d%d%d%d",&h1,&h2,&h3,&h4);
			set_value(h3,h4,get_value(h1,h2));
		} else if(strcmp(order,"SR")==0) {
			scanf("%d%d",&h1,&h2);
			if(rc_flip) {
				if(col_flip){h1=N-h1+1;h2=N-h2+1;}
				temp=col_table[h1];
				col_table[h1]=col_table[h2];
				col_table[h2]=temp;
			} else {
				if(row_flip){h1=N-h1+1;h2=N-h2+1;}
				temp=row_table[h1];
				row_table[h1]=row_table[h2];
				row_table[h2]=temp;
			}
		} else if(strcmp(order,"SC")==0) {
			scanf("%d%d",&h1,&h2);
			if(rc_flip) {
				if(row_flip){h1=N-h1+1;h2=N-h2+1;}
				temp=row_table[h1];
				row_table[h1]=row_table[h2];
				row_table[h2]=temp;
			} else {
				if(col_flip){h1=N-h1+1;h2=N-h2+1;}
				temp=col_table[h1];
				col_table[h1]=col_table[h2];
				col_table[h2]=temp;
			}
		} else if(strcmp(order,"RL")==0) {
			rc_flip=!rc_flip;
			if(rc_flip) {
				col_flip=!col_flip;
			} else {
				row_flip=!row_flip;
			}
		} else if(strcmp(order,"RR")==0) {
			rc_flip=!rc_flip;
			if(rc_flip) {
				row_flip=!row_flip;
			} else {
				col_flip=!col_flip;
			}
		} else if(strcmp(order,"RH")==0) {
			if(rc_flip) {
				col_flip=!col_flip;
			} else {
				row_flip=!row_flip;
			}
		} else if(strcmp(order,"RV")==0) {
			if(rc_flip) {
				row_flip=!row_flip;
			} else {
				col_flip=!col_flip;
			}
		} else return 1;
	}
#ifdef DEBUG_MODE
	for(r=D;r<=E;r++) {
		for(c=F;c<=G;c++) {
			printf("%3d",get_value(r,c));
		}
		puts("");
	}
#endif
	hash=314159265;
	for(r=D;r<=E;r++) {
		for(c=F;c<=G;c++) {
			hash=(31*hash+get_value(r,c))%1000000007;
		}
	}
	printf("%lld\n",hash);
	return 0;
}
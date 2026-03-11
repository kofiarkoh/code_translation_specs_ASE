#include <stdio.h>
#include <stdlib.h>

struct node_t {
	int start, num;
	int leftSum, rightSum;
	struct node_t* left, *right;
};

struct node_t* createNode(int start, int num) {
	struct node_t* node = malloc(sizeof(*node));
	if (node == NULL) exit(2);
	node->start = start;
	node->num = num;
	node->left = node->right = NULL;
	node->leftSum = node->rightSum = 0;
	return node;
}

int getSum(struct node_t* node) {
	if (node == NULL) return 0;
	if (node->leftSum < 0) node->leftSum = getSum(node->left);
	if (node->rightSum < 0) node->rightSum = getSum(node->right);
	return node->num + node->leftSum + node->rightSum;
}

/*
   A
 B    C
# X  Y #

*/

void rotateLeft(struct node_t** target) {
	struct node_t *A, *C, *Y;
	A = *target;
	if (A->right == NULL) exit(3);
	C = A->right;
	Y = C->left;
	A->right = Y; A->rightSum = -1;
	C->left = A; C->leftSum = -1;
	getSum(C);
	*target = C;
}

void rotateRight(struct node_t** target) {
	struct node_t *A, *B, *X;
	A = *target;
	if (A->left == NULL) exit(3);
	B = A->left;
	X = B->right;
	A->left = X; A->leftSum = -1;
	B->right = A; B->rightSum = -1;
	getSum(B);
	*target = B;
}

struct node_t* root;

int getK(int k) {
	struct node_t*** path = NULL;
	int* isLeft = NULL;
	int pathCount = 0;
	struct node_t** current = &root;
	int got = 0;
	int i;
	for (;;) {
		int leftCount = getSum((*current)->left);
		path = realloc(path, sizeof(*path) * (pathCount + 1));
		isLeft = realloc(isLeft, sizeof(*isLeft) * (pathCount + 1));
		if (path == NULL || isLeft == NULL) exit(2);
		path[pathCount] = current;
		if (k <= leftCount) {
			isLeft[pathCount] = 1;
			pathCount++;
			current = &(*current)->left;
		} else if (k <= leftCount + (*current)->num) {
			got = (*current)->start + (k - leftCount - 1);
			break;
		} else {
			k -= leftCount + (*current)->num;
			isLeft[pathCount] = 0;
			pathCount++;
			current = &(*current)->right;
		}
	}
	for (i = pathCount; i > 0; ) {
		if (i == 1) {
			if (isLeft[0]) {
				rotateRight(path[0]);
			} else {
				rotateLeft(path[0]);
			}
			i--;
		} else {
			if (isLeft[i - 2] && isLeft[i - 1]) {
				rotateRight(path[i - 2]);
				rotateRight(path[i - 2]);
			} else if (isLeft[i - 2]) {
				rotateLeft(path[i - 1]);
				rotateRight(path[i - 2]);
			} else if (isLeft[i - 1]) {
				rotateRight(path[i - 1]);
				rotateLeft(path[i - 2]);
			} else {
				rotateLeft(path[i - 2]);
				rotateLeft(path[i - 2]);
			}
			i -= 2;
		}
	}
	free(path);
	free(isLeft);
	return got;
}

int N, Q;
int query[114514][2];

void dump(const struct node_t* node, int indent) {
	int i;
	for (i = 0; i < indent; i++) printf("  ");
	if (node == NULL) {
		puts("NULL");
	} else {
		printf("start=%d num=%d leftSum=%d rightSum=%d\n", node->start, node->num, node->leftSum, node->rightSum);
		dump(node->left, indent + 1);
		dump(node->right, indent + 1);
	}
}

int main(void) {
	int i;
	if (scanf("%d%d", &N, &Q) != 2) return 1;
	for (i = 0; i < Q; i++) {
		if (scanf("%d%d", &query[i][0], &query[i][1]) != 2) return 1;
	}
	root = createNode(1, N);
	for (i = 0; i < Q; i++) {
		int got = getK(query[i][1]);
		if (query[i][0] == 0) {
			printf("%d\n", got);
		} else if (query[i][0] == 2) {
			dump(root, 0);
		} else {
			int offset = query[i][1] - getSum(root->left);
			struct node_t *leftSeq, *rightSeq;
			if (offset >= root->num) {
				leftSeq = root;
				rightSeq = root->right;
				root->right = NULL; root->rightSum = 0;
			} else {
				leftSeq = root;
				rightSeq = createNode(root->start + offset, root->num - offset);
				root->num = offset;
				rightSeq->right = root->right; rightSeq->rightSum = -1;
				leftSeq->right = NULL; leftSeq->rightSum = 0;
			}
			root = createNode(0, 0);
			root->left = rightSeq; root->leftSum = -1;
			root->right = leftSeq; root->rightSum = -1;
		}
	}
	return 0;
}


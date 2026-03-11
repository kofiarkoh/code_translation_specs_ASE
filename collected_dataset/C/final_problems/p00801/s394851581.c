// AOJ 1221 Numoeba
// 2018.2.16 bal4u

#include <stdio.h>
#include <string.h>

#define MAGIC 20

typedef struct { int val; short w, node[MAGIC]; int mk; } T;
T cell[5000]; int top;

int root;
int tm, maxval, maxid, cells, maxcells;

int new(int val, int par)
{
	T *t = cell + top++;
	cells++;
	t->val = val, t->w = 1, t->node[0] = par, t->mk = tm;
	return top-1;
}

int numbosome(int val, int mode)
{
	int k = (val+1)>>1;

	if (!mode) {
		val += k;
		while (!(val & 1)) val >>= 1;
		if (val > 12345678) val -= 12345678;
	} else {
		val = k;
		if (!(val & 1)) val += mode;
	}
	return val;
}

int child(int par, int id)
{
	int i, chi, n = 0;
	for (i = 0; i < cell[id].w; i++) {
		int x = cell[id].node[i];
		if (x == par || cell[x].val == 0) continue;
		n++, chi = x;
	}
	if (n == 0) return 0;
	if (n == 1) return chi;
	return -1;
}

void set_child(int id, int chi)
{
	int i;
	for (i = 0; i < cell[id].w; i++) {
		if (cell[cell[id].node[i]].val == 0) {
			cell[id].node[i] = chi;
			return;
		}
	}
	cell[id].node[cell[id].w++] = chi;
}

void update(int par, int id)
{
	int i, val, chi, leaf_bonus;

	if (cell[id].val == 0 || cell[id].mk == tm) return;
	cell[id].mk = tm;

	chi = child(par, id);

	val = numbosome(cell[id].val, 0);
	if (val > maxval) maxid = id, maxval = val;
	else if (val == maxval) maxid = 0;

	if (val > cell[id].val && chi == 0) {
		leaf_bonus = new(numbosome(val, 1), id);
		set_child(id, leaf_bonus);
	}

	if (val > 1) {
		cell[id].val = val;
		cells++;
		for (i = 0; i < cell[id].w; i++) update(id, cell[id].node[i]);
	} else {
		if (par == 0) return;

		cell[id].val = 0;
		if (chi <= 0) return;

		for (i = 0; ; i++) {
			if (cell[par].node[i] == id) { cell[par].node[i] = chi;	break; }
		}
		for (i = 0; ; i++) {
			if (cell[chi].node[i] == id) { cell[chi].node[i] = par;	break; }
		}
		update(par, chi);
	}
}

int main() 
{
	int k;

	while (scanf("%d", &k) && k) {
		memset(cell, 0, sizeof(cell));
		top = 1, root = top++, maxcells = 1;
		cell[root].val = k;
		for (tm = 1; ; tm++) {
			maxval = 0, maxid = 0, cells = 0;
			update(0, root);
			if (cells == 0) break;
			if (maxid > 0) {
				int leader_bonus;
				root = maxid;
				leader_bonus = new(numbosome(cell[root].val, -1), root);
				set_child(root, leader_bonus);
			}
			if (cells > maxcells) maxcells = cells;
		}
		printf("%d %d\n", tm, maxcells);
	}
	return 0;
}


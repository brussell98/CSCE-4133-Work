#include <iostream>
#include <string>

using namespace std;

// NOTE: Keys may be up to 10^18, so u128 is required

class Node {
public:
	unsigned long long int key;
	bool red;
	int index;
	Node *parent;
	Node *left;
	Node *right;

	Node(unsigned long long int key) {
		this->key = key;
		this->red = true;
		this->index = 1;
		left = right = parent = nullptr;
	}

	// Get the node on the opposite site of this node's parent, or null if none
	Node* sibling() {
		if (this->parent == nullptr)
			return nullptr;

		if (this->parent->left == this)
			return this->parent->right;
		return this->parent->left;
	}
};

class RedBlackTree {
public:
	Node *root;

	// Make X's right child become it's parent, where X is the left child
	void leftRotate(Node *x) {
		Node *y = x->right;

		x->right = y->left;
		if (y->left != nullptr)
			y->left->parent = x;

		y->parent = x->parent;

		if (x->parent == nullptr)
			root = y;
		else if (x == x->parent->left)
			x->parent->left = y;
		else
			x->parent->right = y;

		y->left = x;
		x->parent = y;

		// X becomes the left child of y, so y's tree is no longer on its right. Take y.index out of x.index
		x->index -= y->index;
	}

	// Make X's left child become its parent, where X is the right child
	void rightRotate(Node *x) {
		Node *y = x->left;

		x->left = y->right;
		if (y->right != nullptr)
			y->right->parent = x;

		y->parent = x->parent;

		if (x->parent == nullptr)
			root = y;
		else if (x == x->parent->right)
			x->parent->right = y;
		else
			x->parent->left = y;

		y->right = x;
		x->parent = y;

		// X becomes the right child of Y, so Y gains its index.
		y->index += x->index;
	}

	void insertFixup(Node *node) {
		node->red = true;

		// If the parent node is red, correct the double red problem
		if (node != root && node->parent->parent != nullptr && node->parent->red == true) {
			// If the parent has a sibling that is also red, simply recolor and move up to check the grandparent
			if (node->parent->sibling() != nullptr && node->parent->sibling()->red == true) {
				node->parent->red = false;
				node->parent->sibling()->red = false;
				node->parent->parent->red = true;
				insertFixup(node->parent->parent);
			// Restructure for a parent that is a left child
			} else if (node->parent->parent->left != nullptr && node->parent == node->parent->parent->left) {
				// If node is a right child, a a left-right roation much be done
				if (node == node->parent->right) {
					node = node->parent;
					leftRotate(node);
				}
				node->parent->red = false;
				node->parent->parent->red = true;
				rightRotate(node->parent->parent);
			// Restructure for a parent that is a right child
			} else if (node->parent->parent->right != nullptr && node->parent == node->parent->parent->right) {
				// If the node is a left child, a right-left rotation must be done
				if (node == node->parent->left) {
					node = node->parent;
					rightRotate(node);
				}
				node->parent->red = false;
				node->parent->parent->red = true;
				leftRotate(node->parent->parent);
			}
		}

		root->red = false;
	}

	// Normal binary tree insert
	void insert(unsigned long long int key) {
		Node *node = new Node(key);
		Node *parent = nullptr;
		Node *curr = root;
		while (curr != nullptr) {
			parent = curr;
			if (node->key < curr->key)
				curr = curr->left;
			else {
				parent->index++; // Increment the index, as a node is being inserted into its right subtree
				curr = curr->right;
			}
		}

		node->parent = parent;
		// If no root exists, set it as the root and change the color to black
		if (parent == nullptr) {
			root = node;
			node->red = false;
			return;
		} else if (node->key < parent->key)
			parent->left = node;
		else
			parent->right = node;

		insertFixup(node);
	}

	// Search for the node, adding the current node's index to the sum if traversing left from it
	int index(unsigned long long int key) {
		int sum = 0;
		Node *curr = root;
		while (curr != nullptr) {
			if (key == curr->key)
				return sum + curr->index; // When found return the sum of indexes plus the found node's index
			if (key < curr->key) {
				sum += curr->index;
				curr = curr->left;
			} else
				curr = curr->right;
		}

		return 0; // If the node was not found, return 0
	}
};

int main() {
	string input;
	getline(cin, input);
	int lines = atoi(input.c_str());
	RedBlackTree tree = RedBlackTree{};
	
	for (int i = 0; i < lines; i++) {
		string line;
		getline(cin, line);
		int type = atoi(line.substr(0, 1).c_str());
		unsigned long long int key = strtoull(line.substr(2).c_str(), nullptr, 10);

		if (type == 1)
			tree.insert(key);
		else {
			unsigned long int index = tree.index(key);
			if (index == 0)
				cout << "Data tidak ata" << endl; // Translation: Missing data
			else
				cout << index << endl;
		}
	}

	return 0;
}

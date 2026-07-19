<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_myskill extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_myskill';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_by_uid_petid($uid, $petid) {
        return DB::fetch_all('SELECT * FROM %t WHERE uid=%d AND petid=%d', [$this->_table, $uid, $petid]);
    }

    public function delete_by_petid($petid) {
        DB::query('DELETE FROM %t WHERE petid=%d', [$this->_table, $petid]);
    }
}

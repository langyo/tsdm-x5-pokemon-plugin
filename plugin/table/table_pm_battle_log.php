<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_battle_log extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_battle_log';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_by_uid($uid, $limit = 20) {
        return DB::fetch_all(
            'SELECT * FROM %t WHERE uid_1=%d OR uid_2=%d ORDER BY created_at DESC LIMIT %d',
            [$this->_table, $uid, $uid, $limit]
        );
    }
}
